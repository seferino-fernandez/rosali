<template>
    <ScrollPanel class="p-scrollpanel-content">
        <highlightjs autodetect :code="logs" />
    </ScrollPanel>
</template>

<script>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import ScrollPanel from "primevue/scrollpanel";
import hljs from "highlight.js/lib/common";
import hljsVuePlugin from "@highlightjs/vue-plugin";
import { invoke, event } from "@tauri-apps/api";

export default {
    components: {
        highlightjs: hljsVuePlugin.component,
        ScrollPanel,
    },
    setup(props) {
        const route = useRoute();
        const logs = ref("");
        const clusterConnectionId = ref(route.params.id);
        const podName = ref(route.params.podName);
        const podNamespace = ref(route.params.podNameSpace);

        onMounted(async () => {
            event.listen("log_line", (event) => {
                if (event.payload !== null) {
                    logs.value += event.payload;
                }
            });
            streamLogs();
        });
        const streamLogs = async () => {
            if (!podName.value || !clusterConnectionId.value) {
                return;
            }

            logs.value = "";
            await invoke("stream_pod_logs_command", {
                id: clusterConnectionId.value,
                namespace: podNamespace.value,
                podName: podName.value,
            });
        };

        return {
            logs
        };
    },
};
</script>

<style scoped>
body .p-scrollpanel-content {
    background-color: black;
    color: white;
}
</style>
