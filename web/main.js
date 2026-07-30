/**
 * Paper Studio - Main Application
 * Phase 0: Technical Preview
 */

// State
let editorEl = null;
let previewEl = null;
let statusEl = null;
let parseTimeEl = null;

// Debounce helper
function debounce(fn, delay) {
    let timer = null;
    return function (...args) {
        clearTimeout(timer);
        timer = setTimeout(() => fn.apply(this, args), delay);
    };
}

// Initialize application
function init() {
    editorEl = document.getElementById('editor');
    previewEl = document.getElementById('preview');
    statusEl = document.getElementById('status');
    parseTimeEl = document.getElementById('parse-time');
    const refreshBtn = document.getElementById('refresh-btn');

    statusEl.textContent = 'Ready';

    // Set up event listeners
    editorEl.addEventListener('input', debounce(updatePreview, 150));
    refreshBtn.addEventListener('click', () => updatePreview());

    // Initial render
    updatePreview();
}

// Update preview
function updatePreview() {
    const content = editorEl.value;
    const startTime = performance.now();

    try {
        const html = fallbackParse(content);
        previewEl.innerHTML = html;

        // Render math with KaTeX
        if (typeof renderMathInElement === 'function') {
            renderMathInElement(previewEl, {
                delimiters: [
                    { left: '\\[', right: '\\]', display: true },
                    { left: '\\(', right: '\\)', display: false },
                    { left: '$', right: '$', display: false }
                ],
                throwOnError: false
            });
        }

        const elapsed = (performance.now() - startTime).toFixed(2);
        parseTimeEl.textContent = `Parsed in ${elapsed}ms`;

    } catch (e) {
        console.error('Parse error:', e);
        previewEl.innerHTML = `<div class="error-message">Parse Error: ${e.message}</div>`;
    }
}

// Fallback parser (JavaScript implementation)
function fallbackParse(input) {
    const lines = input.split('\n');
    let html = '<article class="paper">\n';
    let i = 0;

    while (i < lines.length) {
        const line = lines[i].trim();

        if (line === '') {
            i++;
            continue;
        }

        // Section
        if (line.startsWith('@section ')) {
            const title = line.substring(9).trim();
            html += `<h2 class="section-title">${escapeHtml(title)}</h2>\n`;
            i++;
            continue;
        }

        // Subsection
        if (line.startsWith('@subsection ')) {
            const title = line.substring(12).trim();
            html += `<h3 class="section-title">${escapeHtml(title)}</h3>\n`;
            i++;
            continue;
        }

        // Subsubsection
        if (line.startsWith('@subsubsection ')) {
            const title = line.substring(15).trim();
            html += `<h4 class="section-title">${escapeHtml(title)}</h4>\n`;
            i++;
            continue;
        }

        // Figure
        if (line.startsWith('@figure{')) {
            const result = parseBlock(lines, i);
            html += renderFigure(result.attrs);
            i = result.endIndex + 1;
            continue;
        }

        // Table
        if (line.startsWith('@table{')) {
            const result = parseBlock(lines, i);
            html += renderTable(result.attrs);
            i = result.endIndex + 1;
            continue;
        }

        // Equation
        if (line.startsWith('@equation{')) {
            const result = parseBlock(lines, i);
            html += renderEquation(result.attrs);
            i = result.endIndex + 1;
            continue;
        }

        // Skip meta and abstract for now
        if (line.startsWith('@meta{') || line.startsWith('@abstract{')) {
            const result = parseBlock(lines, i);
            i = result.endIndex + 1;
            continue;
        }

        // Paragraph (any other text)
        if (!line.startsWith('@')) {
            html += `<p>${parseInline(line)}</p>\n`;
            i++;
            continue;
        }

        i++;
    }

    html += '</article>\n';
    return html;
}

// Parse a block element (figure, table, equation)
function parseBlock(lines, startIndex) {
    let braceCount = 0;
    let content = '';
    let i = startIndex;

    for (; i < lines.length; i++) {
        const line = lines[i];
        content += line + '\n';

        for (const char of line) {
            if (char === '{') braceCount++;
            if (char === '}') braceCount--;
        }

        if (braceCount === 0) break;
    }

    const attrs = parseAttributes(content);
    return { attrs, endIndex: i };
}

// Parse attributes from block content
function parseAttributes(content) {
    const attrs = {};

    // Match key = "value" patterns
    const stringPattern = /(\w+)\s*=\s*"([^"]*)"/g;
    let match;
    while ((match = stringPattern.exec(content)) !== null) {
        attrs[match[1]] = match[2];
    }

    // Match columns = [...] pattern
    const columnsMatch = content.match(/columns\s*=\s*\[([\s\S]*?)\]/);
    if (columnsMatch) {
        attrs.columns = columnsMatch[1]
            .match(/"([^"]*)"/g)
            ?.map(s => s.replace(/"/g, '')) || [];
    }

    // Match rows = [...] pattern
    const rowsMatch = content.match(/rows\s*=\s*\[([\s\S]*?)\]\s*\}/);
    if (rowsMatch) {
        const rowsContent = rowsMatch[1];
        const rowMatches = rowsContent.match(/\[[^\]]+\]/g) || [];
        attrs.rows = rowMatches.map(row => {
            return row.match(/"([^"]*)"/g)?.map(s => s.replace(/"/g, '')) || [];
        });
    }

    return attrs;
}

// Parse inline elements
function parseInline(text) {
    // Citations: @cite{key}
    text = text.replace(/@cite\{([^}]+)\}/g, '<cite class="citation">[$1]</cite>');
    // References: @ref{target}
    text = text.replace(/@ref\{([^}]+)\}/g, '<a href="#$1" class="reference">$1</a>');
    // Inline math: $...$
    text = text.replace(/\$([^$]+)\$/g, '\\($1\\)');
    return text;
}

// Render figure
function renderFigure(attrs) {
    const id = attrs.label ? ` id="${escapeHtml(attrs.label)}"` : '';
    const caption = attrs.caption ? `<figcaption>${escapeHtml(attrs.caption)}</figcaption>` : '';
    const imgSrc = attrs.path || 'placeholder.png';

    return `<figure${id}>
  <img src="${escapeHtml(imgSrc)}" alt="${escapeHtml(attrs.caption || '')}" onerror="this.style.display='none'">
  ${caption}
</figure>\n`;
}

// Render table
function renderTable(attrs) {
    const id = attrs.label ? ` id="${escapeHtml(attrs.label)}"` : '';
    const caption = attrs.caption ? `<figcaption>${escapeHtml(attrs.caption)}</figcaption>` : '';

    let tableHtml = `<figure class="table-container"${id}>\n${caption}\n<table>\n`;

    // Header
    if (attrs.columns && attrs.columns.length > 0) {
        tableHtml += '<thead><tr>\n';
        for (const col of attrs.columns) {
            tableHtml += `<th>${escapeHtml(col)}</th>\n`;
        }
        tableHtml += '</tr></thead>\n';
    }

    // Body
    if (attrs.rows && attrs.rows.length > 0) {
        tableHtml += '<tbody>\n';
        for (const row of attrs.rows) {
            tableHtml += '<tr>\n';
            for (const cell of row) {
                tableHtml += `<td>${escapeHtml(cell)}</td>\n`;
            }
            tableHtml += '</tr>\n';
        }
        tableHtml += '</tbody>\n';
    }

    tableHtml += '</table>\n</figure>\n';
    return tableHtml;
}

// Render equation
function renderEquation(attrs) {
    const id = attrs.label ? ` id="${escapeHtml(attrs.label)}"` : '';
    const content = attrs.content || '';
    return `<div class="equation"${id}>\n\\[${content}\\]\n</div>\n`;
}

// Escape HTML special characters
function escapeHtml(str) {
    if (!str) return '';
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

// Wait for DOM to load
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
