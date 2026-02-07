import { invoke } from "@tauri-apps/api/core";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import { open as openPath } from "@tauri-apps/plugin-shell";
import { marked } from "marked";

// ---------- 侧边栏：工具包菜单切换 ----------
const navItems = document.querySelectorAll(".sidebar-nav .nav-item[data-tool]");
const panels = document.querySelectorAll(".main-content .tool-panel");

function switchTool(toolId) {
  navItems.forEach((el) => {
    const isActive = el.getAttribute("data-tool") === toolId;
    el.classList.toggle("active", isActive);
    el.setAttribute("aria-current", isActive ? "page" : null);
  });
  panels.forEach((el) => {
    el.classList.toggle("active", el.id === `panel-${toolId}`);
  });
}

navItems.forEach((btn) => {
  btn.addEventListener("click", () => {
    const tool = btn.getAttribute("data-tool");
    if (tool) switchTool(tool);
  });
});

/** 等待 Tauri 注入完成（dev 下脚本可能早于 __TAURI_INTERNALS__） */
function whenTauriReady() {
  return new Promise((resolve) => {
    if (typeof window !== "undefined" && window.__TAURI_INTERNALS__?.invoke) {
      resolve();
      return;
    }
    const t = setInterval(() => {
      if (typeof window !== "undefined" && window.__TAURI_INTERNALS__?.invoke) {
        clearInterval(t);
        resolve();
      }
    }, 50);
    setTimeout(() => {
      clearInterval(t);
      resolve();
    }, 5000);
  });
}

/** 是否在 Tauri 应用内运行（否则为普通浏览器，无文件/系统 API） */
function isTauri() {
  return typeof window !== "undefined" && window.__TAURI_INTERNALS__?.invoke != null;
}

const dropZone = document.getElementById("dropZone");
const selectFileBtn = document.getElementById("selectFileBtn");
const fileNameEl = document.getElementById("fileName");
const convertBtn = document.getElementById("convertBtn");
const messageEl = document.getElementById("message");
const previewContent = document.getElementById("previewContent");
const downloadBtn = document.getElementById("downloadBtn");
const saveAsBtn = document.getElementById("saveAsBtn");

let selectedPath = null;
let lastMarkdown = "";

function setMessage(text, type = "") {
  messageEl.textContent = text;
  messageEl.className = "message " + type;
}

function setPreview(html) {
  if (!html) {
    previewContent.innerHTML = "";
    previewContent.classList.add("empty");
    previewContent.textContent = "转换后将在此处预览 Markdown";
    return;
  }
  previewContent.classList.remove("empty");
  previewContent.innerHTML = html;
}

function updateActions() {
  convertBtn.disabled = !selectedPath;
  const hasContent = lastMarkdown.length > 0;
  downloadBtn.disabled = !hasContent;
  saveAsBtn.disabled = !hasContent;
}

// Markdown 渲染（marked 已在依赖中）
marked.setOptions({ gfm: true });
function renderMarkdown(md) {
  if (!md) return "";
  return marked.parse(md);
}

// 选择文件：使用 Tauri dialog（需在 Tauri 窗口内运行）
async function pickXmindFile() {
  if (!isTauri()) {
    setMessage("请在桌面应用中运行：在项目目录执行 npm run tauri dev", "error");
    return;
  }
  try {
    const path = await openDialog({
      multiple: false,
      filters: [{ name: "XMind", extensions: ["xmind"] }],
    });
    if (path) {
      selectedPath = typeof path === "string" ? path : path.path ?? path;
      if (typeof selectedPath !== "string") selectedPath = null;
      fileNameEl.textContent = selectedPath ? selectedPath.split(/[/\\]/).pop() : "";
      setMessage("");
      updateActions();
    }
  } catch (e) {
    setMessage("选择文件失败: " + (e?.message || e), "error");
  }
}

// 拖拽
dropZone.addEventListener("dragover", (e) => {
  e.preventDefault();
  dropZone.classList.add("drag-over");
});
dropZone.addEventListener("dragleave", () => {
  dropZone.classList.remove("drag-over");
});
dropZone.addEventListener("drop", (e) => {
  e.preventDefault();
  dropZone.classList.remove("drag-over");
  const file = e.dataTransfer?.files?.[0];
  if (!file) return;
  if (!file.name?.toLowerCase().endsWith(".xmind")) {
    setMessage("请选择 .xmind 文件", "error");
    return;
  }
  // In Tauri webview, dropped file may have .path on desktop
  if (file.path) {
    selectedPath = file.path;
    fileNameEl.textContent = file.name;
    setMessage("");
    updateActions();
  } else {
    setMessage("请使用「选择本地文件」选择 .xmind 文件", "error");
  }
});

selectFileBtn.addEventListener("click", pickXmindFile);

// 转换
convertBtn.addEventListener("click", async () => {
  if (!selectedPath) return;
  if (!isTauri()) {
    setMessage("请在桌面应用中运行：npm run tauri dev", "error");
    return;
  }
  setMessage("转换中…");
  try {
    const md = await invoke("convert_xmind_to_markdown", { path: selectedPath });
    lastMarkdown = md ?? "";
    setPreview(renderMarkdown(lastMarkdown));
    setMessage("转换成功", "success");
    updateActions();
  } catch (e) {
    setMessage("转换失败: " + (e?.message || String(e)), "error");
    lastMarkdown = "";
    setPreview("");
    updateActions();
  }
});

// 下载 / 另存为：写文件
async function saveMarkdownToPath(path, openAfterSave = false) {
  if (!path || !lastMarkdown) return;
  if (!isTauri()) {
    setMessage("请在桌面应用中运行：npm run tauri dev", "error");
    return;
  }
  try {
    await invoke("save_markdown_to_file", { path, content: lastMarkdown });
    setMessage("已保存: " + path, "success");
    if (openAfterSave) {
      try {
        await openPath(path);
      } catch (_) {}
    }
  } catch (e) {
    setMessage("保存失败: " + (e?.message || String(e)), "error");
  }
}

downloadBtn.addEventListener("click", async () => {
  if (!isTauri()) {
    setMessage("请在桌面应用中运行：npm run tauri dev", "error");
    return;
  }
  const defaultName = selectedPath
    ? selectedPath.replace(/\.xmind$/i, ".md").split(/[/\\]/).pop()
    : "output.md";
  try {
    const path = await saveDialog({
      defaultPath: defaultName,
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });
    if (path) {
      const p = typeof path === "string" ? path : path.path ?? path;
      await saveMarkdownToPath(p, true);
    }
  } catch (e) {
    setMessage("保存失败: " + (e?.message || e), "error");
  }
});

saveAsBtn.addEventListener("click", async () => {
  if (!isTauri()) {
    setMessage("请在桌面应用中运行：npm run tauri dev", "error");
    return;
  }
  try {
    const path = await saveDialog({
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });
    if (path) await saveMarkdownToPath(typeof path === "string" ? path : path.path ?? path);
  } catch (e) {
    setMessage("另存为失败: " + (e?.message || e), "error");
  }
});

// 初始化：等待 Tauri 就绪后再启用界面（避免 invoke 未注入时报错）
(async () => {
  await whenTauriReady();
  setPreview("");
  updateActions();
})();
