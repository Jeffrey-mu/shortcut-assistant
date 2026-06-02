const editableSelector = 'input, textarea, select, [contenteditable="true"], [contenteditable=""]';

export const isTauriRuntime = () => Boolean((window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__);

const isEditableTarget = (target: EventTarget | null) => {
  if (!(target instanceof Element)) return false;
  return Boolean(target.closest(editableSelector));
};

const isBrowserShortcut = (event: KeyboardEvent) => {
  if (event.key === 'F5') return true;
  if (!(event.metaKey || event.ctrlKey)) return false;

  const key = event.key.toLowerCase();
  return [
    'a',
    'f',
    'g',
    'p',
    'r',
    's',
    'u',
    '=',
    '+',
    '-',
    '0',
  ].includes(key);
};

export const installBrowserGuards = () => {
  window.addEventListener('selectstart', (event) => {
    if (!isEditableTarget(event.target)) {
      event.preventDefault();
    }
  });

  window.addEventListener('dragstart', (event) => {
    if (!isEditableTarget(event.target)) {
      event.preventDefault();
    }
  });

  window.addEventListener('contextmenu', (event) => {
    if (!isEditableTarget(event.target)) {
      event.preventDefault();
    }
  });

  window.addEventListener('keydown', (event) => {
    if (isEditableTarget(event.target)) return;
    if (isBrowserShortcut(event)) {
      event.preventDefault();
      event.stopPropagation();
    }
  });
};
