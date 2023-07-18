<template>
  <div class="flex flex-column w-full h-full">
    <AppTopbar :tabs="tabs" />
    <div class="flex flex-grow mt-7">
      <AppSidebar v-show="showSidebar" />
      <AppMainView @context-selected="addClusterTab" @view-logs="addLogTab" />
      <AppDetailsSidebar v-show="showDetailsSidebar" :resource="selectedResource" :hideDetails="hideDetails"
        :showDetailsSidebar="showDetailsSidebar" />
    </div>
  </div>
</template>

<script setup>
import AppTopbar from "./AppTopbar.vue";
import AppSidebar from "./AppSidebar.vue";
import AppMainView from "./AppMainView.vue";
import AppDetailsSidebar from "./AppDetailsSidebar.vue";
import { ref, computed, provide } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";

const TAB_TYPE_CLUSTER = 'cluster';
const tabs = ref([]);
const selectedResource = ref({});
const router = useRouter();
const route = useRoute();
const connectionId = ref(null);
const showSidebar = computed(() => route.name !== "KubeconfigContexts");
const showDetailsSidebar = ref(false);

async function addClusterTab(path, label, name, params) {
  const tabIndex = tabs.value.findIndex((t) => {
    return label === t.label && path === t.path;
  });

  if (tabIndex > -1) {
    router.push(tabs.value[tabIndex]);
  } else {
    let clusterConnectionResponse = await invoke("add_cluster_connection", { "contextName": label, "contextPath": path });
    let id = clusterConnectionResponse.data.connection_id;
    let namespace = clusterConnectionResponse.data.namespace;
    params.id = id;
    let newTab = { type: TAB_TYPE_CLUSTER, id, path, label, name, params };
    tabs.value.push(newTab);
    router.push({ name, params, query: { namespace } });
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
}

async function displayDetails(resource) {
  selectedResource.value = resource;
  showDetailsSidebar.value = true;
}

async function hideDetails() {
  showDetailsSidebar.value = false;
}

provide("addClusterTab", addClusterTab);
provide("displayDetails", displayDetails);
provide("hideDetails", hideDetails);
</script>
