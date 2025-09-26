document.addEventListener("DOMContentLoaded", () => {
    const panel = document.createElement("div");
    panel.id = "OptionPanel";
    panel.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      width: 10px;
      height: 100px;
      background: rgba(0, 0, 0, 0.2) !important;
      z-index: 2147483647;
      transition: width 0.3s ease;
      cursor: pointer;
      overflow: hidden;
      display: flex;
      align-items: center;
      font-family: -apple-system, BlinkMacSystemFont, sans-serif;
    `;
    document.body.appendChild(panel);

    const fullWidth = 200;

    panel.setAttribute('data-tauri-drag-region', '');

    panel.addEventListener("mouseenter", () => {
        panel.style.width = `${fullWidth}px`;
        ensurePanelContent();
    });

    panel.addEventListener("mouseleave", () => {
        panel.style.width = "10px";
    });


    function ensurePanelContent() {
        if (panel.querySelector('.panel-content')) return;

        const container = document.createElement("div");
        container.className = 'panel-content';
        container.style.cssText = `
        padding: 0 10px;
        height: 100%;
        display: flex;
        align-items: center;
        gap: 12px;
        color: white;
      `;
        panel.appendChild(container);
    }

    const observer = new MutationObserver(() => {
        const container = panel.querySelector('.panel-content');
        if (!container) return;

        const items = document.querySelectorAll('.readerControls_item');
        if (items.length > 0 && !container.contains(items[items.length - 1])) {
            container.appendChild(items[items.length - 1]);
        }
    });

    observer.observe(document.body, { childList: true, subtree: true });
});