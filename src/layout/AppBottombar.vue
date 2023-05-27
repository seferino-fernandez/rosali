<template>
    <TabView v-model="activeTabIndex">
        <TabPanel v-for="(tab, index) in podLogTabs" :key="tab.id" :header="tab.pod.name">
            <ScrollPanel class="p-scrollpanel-content" style="width: 100%; height: 100%">
                <highlightjs autodetect :code="tab.logs" />
            </ScrollPanel>
            <Button icon="pi pi-times" class="close-tab-button" @click="closeTab(index)" />
        </TabPanel>
    </TabView>
</template>
  
<script>
import { ref, inject, watch, computed, onMounted } from "vue";
import ScrollPanel from "primevue/scrollpanel";
import Button from "primevue/button";
import hljs from "highlight.js/lib/common";
import hljsVuePlugin from "@highlightjs/vue-plugin";
import { invoke, event } from "@tauri-apps/api";
import TabView from "primevue/tabview";
import TabPanel from "primevue/tabpanel";

export default {
    components: {
        Button,
        ScrollPanel,
        highlightjs: hljsVuePlugin.component,
        TabView,
        TabPanel,
    },
    setup() {
        const logs = ref("");
        const podLogTabs = ref([]);
        const bottombarData = inject("bottombarData");
        const connectionId = bottombarData.connectionId;
        const selectedPod = bottombarData.selectedPod;
        const activeTabIndex = ref(0);

        const watchSource = computed(() => [connectionId.value, selectedPod.value]);

        watch(watchSource, () => {
            streamLogs();
        });

        onMounted(async () => {
            event.listen("log_line", (event) => {
                console.log("Received event for log_line:", event);
                if (event.payload !== null && podLogTabs.value.length > 0) {
                    podLogTabs.value[activeTabIndex.value].logs += event.payload;
                }
            });
        });

        const streamLogs = async () => {
            if (!selectedPod.value.name || !connectionId.value) {
                console.log("Not streaming logs");
                return;
            }
            console.log("streamLogs function executing for:", selectedPod.value.name);

            logs.value = ""; // Clear the previous logs
            console.log(selectedPod.value);
            await invoke("stream_pod_logs_command", {
                id: connectionId.value,
                namespace: selectedPod.value.namespace,
                podName: selectedPod.value.name,
            });

            // Add a new tab for each new streaming request
            podLogTabs.value.push({
                id: connectionId.value,
                pod: selectedPod.value,
                logs: logs.value,
            });
            activeTabIndex.value = podLogTabs.value.length - 1;
        };

        const closeTab = (index) => {
            podLogTabs.value.splice(index, 1);
            if (podLogTabs.value.length > 0) {
                activeTabIndex.value = podLogTabs.value.length - 1;
            } else {
                activeTabIndex.value = null;
            }
        };

        return {
            logs,
            streamLogs,
            connectionId,
            selectedPod,
            podLogTabs,
            activeTabIndex,
            closeTab,
        };
    },
};
</script>

<style scoped>
.p-scrollpanel-content {
    background-color: black;
    color: white;
}
</style>