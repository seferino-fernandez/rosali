<template>
  <div class="flex flex-column max-w-screen max-h-screen">
    <AppTopbar :tabs="tabs" />
    <div class="flex flex-inline">
      <div class="flex-none">
        <AppSidebar v-show="showSidebar" />
      </div>
      <div class="flex-grow-1">
        <AppView @context-selected="addClusterTab" @view-logs="addLogTab" />
      </div>
    </div>
  </div>
</template>

<script>
import AppTopbar from "./AppTopbar.vue";
import AppSidebar from "./AppSidebar.vue";
import AppView from "./AppView.vue";
import { ref, computed, provide } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";

export default {
  components: {
    AppTopbar,
    AppSidebar,
    AppView,
  },
  setup() {
    const tabs = ref([]);
    const router = useRouter();
    const route = useRoute();
    const connectionId = ref(null);
    const TAB_TYPE_CLUSTER = 'cluster';

    async function addClusterTab(path, label, name, params) {
      const tabIndex = tabs.value.findIndex((t) => {
        return label === t.label && path === t.path;
      });

      if (tabIndex > -1) {
        router.push(tabs.value[tabIndex]);
      } else {
        let clusterConnectionResponse = await invoke("add_cluster_connection", { "contextName": label, "contextPath": path });
        let id = clusterConnectionResponse.data;
        params.id = id;
        let newTab = { type: TAB_TYPE_CLUSTER, id, path, label, name, params, };
        tabs.value.push(newTab);
        router.push(newTab);
      }
    }

    const addLogTab = async ({ connectionId: connId, pod }) => {
      connectionId.value = connId;
      tabs.value.push({
        type: 'log',
        label: pod.name,
        name: "PodLogsView",
        params: { id: connId, podNamespace: pod.namespace, podName: pod.name },
      });
    };

    const showSidebar = computed(() => route.name !== "KubeconfigContexts");

    provide("addClusterTab", addClusterTab);

    return {
      tabs,
      showSidebar,
      addClusterTab,
      connectionId,
      addLogTab,
    };
  },
};
</script>

<style scoped></style>
