<template>
  <div id="app-container">
    <AppTopbar :tabs="tabs" />
    <div id="content-container">
      <AppSidebar v-show="showSidebar" />
      <Splitter layout="vertical" id="main-content">
        <SplitterPanel class="flex align-items-center justify-content-center" :size="80">
          <AppView @context-selected="addTab" @view-logs="onViewLogs" />
        </SplitterPanel>
        <SplitterPanel v-if="showBottomBar" :size="40">
          <AppBottombar @view-logs="onViewLogs" />
        </SplitterPanel>
      </Splitter>
    </div>
  </div>
</template>

<script>
import AppTopbar from "./AppTopbar.vue";
import AppSidebar from "./AppSidebar.vue";
import AppBottombar from "./AppBottombar.vue";
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
    AppBottombar,
    AppView,
    Splitter,
    SplitterPanel
  },
  setup() {
    const tabs = ref([]);
    const router = useRouter();
    const route = useRoute();
    const selectedPod = ref({});
    const connectionId = ref(null);
    const showBottomBar = ref(false);
    const bottombarData = ref({});

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
      selectedPod.value = pod;
      showBottomBar.value = true;
      bottombarData.value = { connectionId: connId, selectedPod: pod };
    };

    const showLogs = computed(() => route.name === "PodsOverview");
    const showSidebar = computed(() => route.name !== "KubeconfigContexts");

    provide("addTab", addTab);
    provide('showLogs', showLogs);
    provide("bottombarData", {
      connectionId,
      selectedPod,
    });

    return {
      tabs,
      showSidebar,
      showBottomBar,
      addTab,
      showLogs,
      selectedPod,
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
