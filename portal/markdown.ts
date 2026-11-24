function escapeHtml(raw: string): string {
  return raw
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

export type ImageResolver = (url: string) => string | null;

function renderInline(text: string, resolveImage?: ImageResolver): string {
  // Extract inline code spans first so formatting inside `...` isn't touched.
  const codePlaceholders: string[] = [];
  let tokenized = text.replace(/`([^`]+)`/g, (_m, code) => {
    const idx = codePlaceholders.length;
    codePlaceholders.push(`<code>${escapeHtml(code)}</code>`);
    return `\u0000CODE${idx}\u0000`;
  });

  // Extract images ![alt](src) before escaping HTML or matching links
  const imagePlaceholders: string[] = [];
  tokenized = tokenized.replace(/!\[([^\]]*)\]\(([^)\s]+)\)/g, (_m, altRaw, srcRaw) => {
    const idx = imagePlaceholders.length;
    const alt = escapeHtml(altRaw);
    const resolved = resolveImage?.(srcRaw) ?? srcRaw;
    const safeSrc = escapeHtml(resolved);
    const caption = alt
      ? `<figcaption class="text-[11px] opacity-70 italic mt-1">${alt}</figcaption>`
      : '';
    imagePlaceholders.push(
      `<figure class="my-3"><img src="${safeSrc}" alt="${alt}" class="rounded-xl max-h-96 border border-[#e5e5e0] dark:border-[#2d2d2a] shadow-sm object-contain" />${caption}</figure>`
    );
    return `\u0000IMG${idx}\u0000`;
  });

  tokenized = escapeHtml(tokenized);

  // Links [text](href)
  tokenized = tokenized.replace(
    /\[([^\]]+)\]\(([^)\s]+)\)/g,
    '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>'
  );
  // Bold + italic
  tokenized = tokenized.replace(/\*\*\*([^*]+)\*\*\*/g, '<strong><em>$1</em></strong>');
  tokenized = tokenized.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
  tokenized = tokenized.replace(/\*([^*]+)\*/g, '<em>$1</em>');
  tokenized = tokenized.replace(/(^|\s)_([^_]+)_(\s|$|[.,!?;:])/g, '$1<em>$2</em>$3');
  tokenized = tokenized.replace(/~~([^~]+)~~/g, '<del>$1</del>');

  tokenized = tokenized.replace(/\u0000IMG(\d+)\u0000/g, (_m, idx) => imagePlaceholders[Number(idx)] ?? '');
  return tokenized.replace(/\u0000CODE(\d+)\u0000/g, (_m, idx) => codePlaceholders[Number(idx)] ?? '');
}

export function renderMarkdownHtml(md: string, resolveImage?: ImageResolver): string {
  const lines = md.replace(/\r\n/g, '\n').split('\n');
  const html: string[] = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];

    // Fenced code block (``` or ~~~)
    const fenceMatch = line.trimStart().match(/^(```|~~~)(.*)$/);
    if (fenceMatch) {
      const fence = fenceMatch[1];
      const lang = fenceMatch[2].trim();
      const codeLines: string[] = [];
      i++;
      while (i < lines.length && !lines[i].trimStart().startsWith(fence)) {
        codeLines.push(escapeHtml(lines[i]));
        i++;
      }
      i++;
      html.push(`<pre><code class="language-${escapeHtml(lang)}">${codeLines.join('\n')}</code></pre>`);
      continue;
    }

    // Horizontal rule
    if (/^\s*([-*_])(\s*\1){2,}\s*$/.test(line)) {
      html.push('<hr />');
      i++;
      continue;
    }

    // ATX Headings (#..######)
    const hMatch = line.match(/^(#{1,6})\s+(.*)$/);
    if (hMatch) {
      const level = hMatch[1].length;
      html.push(`<h${level}>${renderInline(hMatch[2].replace(/\s+#+\s*$/, ''), resolveImage)}</h${level}>`);
      i++;
      continue;
    }

    // GFM Table
    if (
      line.includes('|') &&
      i + 1 < lines.length &&
      /^\s*\|?(\s*:?-+:?\s*\|)+\s*(:?-+:?\s*)?\|?\s*$/.test(lines[i + 1])
    ) {
      const parseCells = (row: string) =>
        row
          .trim()
          .replace(/^\|/, '')
          .replace(/\|$/, '')
          .split('|')
          .map((c) => renderInline(c.trim(), resolveImage));

      const headers = parseCells(line);
      i += 2;
      const bodyRows: string[][] = [];
      while (i < lines.length && lines[i].includes('|') && lines[i].trim() !== '') {
        bodyRows.push(parseCells(lines[i]));
        i++;
      }
      html.push(
        `<div class="overflow-x-auto my-2"><table><thead><tr>${headers
          .map((h) => `<th>${h}</th>`)
          .join('')}</tr></thead><tbody>${bodyRows
          .map((r) => `<tr>${r.map((c) => `<td>${c}</td>`).join('')}</tr>`)
          .join('')}</tbody></table></div>`
      );
      continue;
    }

    // Blockquote
    if (line.trimStart().startsWith('>')) {
      const quoteLines: string[] = [];
      while (i < lines.length && lines[i].trimStart().startsWith('>')) {
        quoteLines.push(renderInline(lines[i].replace(/^\s*>\s?/, ''), resolveImage));
        i++;
      }
      html.push(`<blockquote><p>${quoteLines.join('<br />')}</p></blockquote>`);
      continue;
    }

    // Lists (ordered / unordered / task lists, including nested indented sub-items)
    if (/^\s*([-*+]|\d+\.)\s+/.test(line)) {
      const isOrdered = /^\s*\d+\.\s+/.test(line);
      const tag = isOrdered ? 'ol' : 'ul';
      const items: string[] = [];
      while (i < lines.length && /^\s*([-*+]|\d+\.)\s+/.test(lines[i])) {
        const indentMatch = lines[i].match(/^(\s*)/);
        const indent = indentMatch ? indentMatch[1].length : 0;
        const rawItem = lines[i].replace(/^\s*([-*+]|\d+\.)\s+/, '');
        const task = rawItem.match(/^\[( |x|X)\]\s+(.*)$/);
        const nestedStyle = indent >= 2 ? ' class="ml-5"' : '';
        if (task) {
          const checked = task[1].toLowerCase() === 'x' ? 'checked' : '';
          items.push(
            `<li class="list-none ${indent >= 2 ? 'ml-5' : ''}"><input type="checkbox" disabled ${checked} class="mr-1.5" />${renderInline(task[2], resolveImage)}</li>`
          );
        } else {
          items.push(`<li${nestedStyle}>${renderInline(rawItem, resolveImage)}</li>`);
        }
        i++;
      }
      html.push(`<${tag}>${items.join('')}</${tag}>`);
      continue;
    }

    // Blank line
    if (line.trim() === '') {
      i++;
      continue;
    }

    // Paragraph
    const paraLines: string[] = [];
    while (
      i < lines.length &&
      lines[i].trim() !== '' &&
      !/^(#{1,6})\s+/.test(lines[i]) &&
      !/^\s*(```|~~~)/.test(lines[i]) &&
      !lines[i].trimStart().startsWith('>') &&
      !/^\s*([-*_])(\s*\1){2,}\s*$/.test(lines[i]) &&
      !/^\s*([-*+]|\d+\.)\s+/.test(lines[i])
    ) {
      paraLines.push(renderInline(lines[i], resolveImage));
      i++;
    }
    html.push(`<p>${paraLines.join(' ')}</p>`);
  }

  return html.join('\n');
}
