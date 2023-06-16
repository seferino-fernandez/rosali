<template>
  <div id="app-container">
    <AppTopbar :tabs="tabs" :logTabs="logTabs" />
    <div id="content-container">
      <AppSidebar v-show="showSidebar" />
      <Splitter layout="vertical" id="main-content">
        <SplitterPanel class="flex align-items-center justify-content-center" :size="100">
          <AppView @context-selected="addTab" @view-logs="onViewLogs" />
        </SplitterPanel>
      </Splitter>
    </div>
  </div>
</template>

<script>
import AppTopbar from "./AppTopbar.vue";
import AppSidebar from "./AppSidebar.vue";
import AppView from "./AppView.vue";
import { ref, computed, provide } from "vue";
import { useRouter, useRoute } from "vue-router";
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import { invoke } from "@tauri-apps/api";

export default {
  components: {
    AppTopbar,
    AppSidebar,
    AppView,
    Splitter,
    SplitterPanel
  },
  setup() {
    const tabs = ref([]);
    const logTabs = ref([]);
    const router = useRouter();
    const route = useRoute();
    const connectionId = ref(null);

    async function addTab(path, label, name, params) {
      const tabIndex = tabs.value.findIndex((t) => {
        return label === t.label && path === t.path;
      });

      if (tabIndex > -1) {
        router.push(tabs.value[tabIndex]);
      } else {
        let clusterConnectionResponse = await invoke("add_cluster_connection", { "contextName": label, "contextPath": path });
        let id = clusterConnectionResponse.data;
        params.id = id;
        let newTab = { id, path, label, name, params, };
        tabs.value.push(newTab);
        router.push(newTab);
      }
    }

    const onViewLogs = async ({ connectionId: connId, pod }) => {
      connectionId.value = connId;
      logTabs.value.push({
        label: pod.name,
        name: "PodLogsView",
        params: { id: connId, podNamespace: pod.namespace, podName: pod.name },
      });
    };

    const showSidebar = computed(() => route.name !== "KubeconfigContexts");

    provide("addTab", addTab);

    return {
      tabs,
      logTabs,
      showSidebar,
      addTab,
      connectionId,
      onViewLogs,
    };
  },
};
</script>

<style scoped>
#content-container {
  display: flex;
  flex-direction: row;
  height: 100%;
}

#app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

#main-content {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  width: 100%;
}
</style>
