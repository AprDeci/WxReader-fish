document.addEventListener("DOMContentLoaded", () => {
    // 创建悬浮面板
    const panel = document.createElement("div");
    panel.id = "OptionPanel";
    document.body.appendChild(panel);
    const readerControlsItems = document.querySelectorAll("readerControls_item")

    panel.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      width: 10px;
      height: 100px;
      background: rgba(0, 0, 0, 0.6) !important;
      z-index: 2147483647;
      transition: width 0.3s ease;
      cursor: pointer;
      overflow: hidden;
      display: flex;
      align-items: center;
      font-family: -apple-system, BlinkMacSystemFont, sans-serif;
    `;

    const fullWidth = 200;

    panel.addEventListener("mouseenter", () => {
        panel.style.width = `${fullWidth}px`;

        if (!panel.innerHTML) {
            const container = document.createElement("div");
            container.style.cssText = `
          padding: 0 10px;
          height: 100%;
          display: flex;
          align-items: center;
          gap: 12px;
          color: white;
        `;

            const themeBtn = document.createElement("button");
            themeBtn.textContent = "🌓 切换主题";
            themeBtn.style.cssText = `
          background: rgba(255, 255, 255, 0.2);
          border: none;
          color: white;
          padding: 4px 10px;
          border-radius: 4px;
          cursor: pointer;
          font-size: 14px;
          backdrop-filter: blur(2px);
        `;

            themeBtn.addEventListener("click", (e) => {
                e.stopPropagation();
                const html = document.documentElement;
                const current = html.getAttribute('theme-mode');
                if (current === 'dark') {
                    html.setAttribute('theme-mode', 'light');
                } else {
                    html.setAttribute('theme-mode', 'dark');
                }
            });

            container.appendChild(themeBtn);
            panel.appendChild(container);
            readerControlsItems.forEach(item => {
                panel.appendChild(item);
            });
        }
    });

    panel.addEventListener("mouseleave", () => {
        panel.style.width = "10px";
    });
});