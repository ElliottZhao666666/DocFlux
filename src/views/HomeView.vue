<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { storeToRefs } from "pinia";
import { useAppStore } from "../stores/app";

const appStore = useAppStore();
const { backendMessage, backendStatus } = storeToRefs(appStore);

/**
 * 在首页加载完成后主动调用后端测试接口。
 * 这样可以在第一阶段快速验证 Tauri invoke 通信链路是否可用。
 */
async function checkBackendStatus() {
  try {
    const response = await invoke<string>("greet_docflux");
    appStore.setBackendState("通信正常", response);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    appStore.setBackendState("通信失败", message);
  }
}

onMounted(async () => {
  await checkBackendStatus();
});
</script>

<template>
  <section class="min-h-screen px-6 py-12 text-slate-100">
    <div class="mx-auto flex min-h-[calc(100vh-6rem)] max-w-5xl items-center justify-center">
      <div class="w-full rounded-3xl border border-slate-800/80 bg-slate-950/60 p-10 shadow-2xl shadow-slate-950/40 backdrop-blur">
        <div class="space-y-6">
          <span class="inline-flex rounded-full border border-cyan-400/30 bg-cyan-400/10 px-4 py-1 text-sm text-cyan-300">
            DocFlux · Phase 1 Bootstrap
          </span>
          <div class="space-y-3">
            <h1 class="text-3xl font-bold tracking-tight md:text-5xl">
              胖哥文溯 DocFlux - 核心引擎已启动
            </h1>
            <p class="max-w-3xl text-base leading-7 text-slate-300 md:text-lg">
              当前页面用于验证桌面端基础脚手架、前后端通信链路与核心数据目录初始化流程是否已成功接通。
            </p>
          </div>

          <div class="grid gap-4 md:grid-cols-2">
            <div class="rounded-2xl border border-slate-800 bg-slate-900/80 p-5">
              <p class="text-sm text-slate-400">后端通信状态</p>
              <p class="mt-3 text-2xl font-semibold text-emerald-300">
                {{ backendStatus }}
              </p>
            </div>
            <div class="rounded-2xl border border-slate-800 bg-slate-900/80 p-5">
              <p class="text-sm text-slate-400">后端返回消息</p>
              <p class="mt-3 break-all text-sm leading-6 text-slate-200">
                {{ backendMessage || "尚未收到响应" }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
