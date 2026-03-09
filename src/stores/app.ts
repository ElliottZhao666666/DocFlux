import { defineStore } from "pinia";
import { ref } from "vue";

/**
 * 全局应用状态。
 * 当前阶段仅保留与核心引擎连通性相关的状态，避免过早设计复杂结构。
 */
export const useAppStore = defineStore("app", () => {
  const backendStatus = ref("等待检测");
  const backendMessage = ref("");

  function setBackendState(status: string, message: string) {
    backendStatus.value = status;
    backendMessage.value = message;
  }

  return {
    backendStatus,
    backendMessage,
    setBackendState,
  };
});
