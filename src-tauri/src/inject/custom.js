document.addEventListener("DOMContentLoaded", () => {
  if (document.getElementById("OptionPanel")) return;

  const panel = document.createElement("div");
  panel.id = "OptionPanel";
  panel.style.cssText = `
      position: fixed;
      top: 50%;
      left: 0;
      transform: translateY(-50%);
      width: 8px;
      height: 200px;
      background: rgba(30, 30, 35, 0.75);
      backdrop-filter: blur(10px);
      -webkit-backdrop-filter: blur(10px);
      border-radius: 0 8px 8px 0;
      z-index: 2147483647;
      transition: width 0.3s cubic-bezier(0.2, 0, 0.2, 1);
      cursor: pointer;
      overflow: hidden;
      display: flex;
      align-items: center;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      box-shadow: 2px 0 12px rgba(0, 0, 0, 0.25);
      border: 1px solid rgba(255, 255, 255, 0.08);
    `;
  panel.setAttribute("data-tauri-drag-region", "");
  document.body.appendChild(panel);

  const fullWidth = 320;

  panel.addEventListener("mouseenter", () => {
    panel.style.width = `${fullWidth}px`;
    ensurePanelContent();
  });

  panel.addEventListener("mouseleave", () => {
    panel.style.width = "8px";
  });

  function ensurePanelContent() {
    if (panel.querySelector(".panel-content")) return;

    const container = document.createElement("div");
    container.className = "panel-content";
    container.style.cssText = `
        padding: 0 16px;
        
        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 14px;
        color: white;
      `;

    //拖动框
    const dragRegion = document.createElement("div");
    dragRegion.setAttribute("data-tauri-drag-region", "");
    dragRegion.style.cssText = `
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        cursor: move;
        z-index: -1;
      `;
    container.appendChild(dragRegion);

    // 翻页按钮
    const toggleBtn = document.createElement("button");
    toggleBtn.innerHTML = "👁️ 显示/隐藏翻页按钮";
    toggleBtn.style.cssText = `
        background: rgba(255, 255, 255, 0.12);
        border: 1px solid rgba(255, 255, 255, 0.15);
        color: white;
        padding: 8px 14px;
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        backdrop-filter: blur(4px);
        transition: all 0.2s ease;
        text-align: left;
        width: 100%;
      `;
    toggleBtn.addEventListener("mouseenter", () => {
      toggleBtn.style.background = "rgba(255, 255, 255, 0.2)";
    });
    toggleBtn.addEventListener("mouseleave", () => {
      toggleBtn.style.background = "rgba(255, 255, 255, 0.12)";
    });
    toggleBtn.addEventListener("click", (e) => {
      e.stopPropagation();
      const target = document.querySelector(".renderTarget_pager");
      if (target) {
        const isVisible = target.style.opacity !== "0";
        target.style.opacity = isVisible ? "0" : "1";
        target.style.pointerEvents = isVisible ? "none" : "auto";
        toggleBtn.textContent = isVisible
          ? "👁️ 显示翻页按钮"
          : "✅ 翻页按钮已显示";
        setTimeout(() => {
          toggleBtn.textContent = "👁️ 显示/隐藏翻页按钮";
        }, 1000);
      }
    });

    container.appendChild(toggleBtn);

    //返回首页按钮
    const indexBtn = document.createElement("button");
    indexBtn.style.cssText = `
        background: rgba(255, 255, 255, 0.12);
        border: 1px solid rgba(255, 255, 255, 0.15);
        color: white;
        padding: 8px 14px;
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        backdrop-filter: blur(4px);
        transition: all 0.2s ease;
        text-align: left;
        width: 100%;
      `;
    indexBtn.innerHTML = "🏠 返回首页";
    indexBtn.addEventListener("click", () => {
      if (document.querySelector(".readerTopBar_link")) {
        document.querySelector(".readerTopBar_link").click();
      }
    });

    container.appendChild(indexBtn);

    //字体颜色选择器
    const fontColorPicker = document.createElement("input");
    fontColorPicker.type = "color";
    const currentColor = document.querySelector(".readerChapterContent").style.color;
    fontColorPicker.value = currentColor;
    container.appendChild(fontColorPicker);
    fontColorPicker.addEventListener("input", () => {
      document.querySelector(".readerChapterContent").style.color =
        fontColorPicker.value;
    });

    panel.appendChild(container);
  }

  // 可选：自动注入其他控件（保留你的逻辑）
  const observer = new MutationObserver(() => {
    const container = panel.querySelector(".panel-content");
    if (!container) return;

    const items = document.querySelectorAll(".readerControls_item");
    if (items.length > 0 && !container.contains(items[items.length - 1])) {
      items[items.length - 1].style = `
        width: 100%;
        padding: 8px 14px;
        height: 40px;
        border: 1px solid rgba(255, 255, 255, 0.15);
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        backdrop-filter: blur(4px);
        transition: all 0.2s ease;
        text-align: left;
      `;
      container.appendChild(items[items.length - 1]);
    }
  });

  observer.observe(document.body, { childList: true, subtree: true });
});
