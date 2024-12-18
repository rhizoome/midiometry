declare global {
    interface Window {

      ipc: { postMessage: (message: string) => void };
      // when the plugin backend (audio thread) sends a message to the GUI thread
      onPluginMessage: (message: number[]) => void;
    }
  }

  export function sendToPlugin(msg: string) {
    window.ipc.postMessage(JSON.stringify(msg));
  }