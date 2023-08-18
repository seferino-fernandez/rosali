<template>
    <div class="flex surface-ground p-5 w-3">
        <pre><code>{{ logs }}</code></pre>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { invoke, event } from "@tauri-apps/api";

const route = useRoute();
const logs = ref("");
const urlParams = new URLSearchParams(window.location.search);

onMounted(async () => {
    event.listen("log_line", (event) => {
        if (event.payload !== null) {
            logs.value += event.payload;
        }
    });
    streamLogs();
});

const streamLogs = async () => {
    console.log("Streaming logs start");
    console.log(urlParams.get("id"));
    console.log(urlParams.get("podName"));
    console.log(urlParams.get("podNamespace"));
    logs.value = "";
    await invoke("stream_pod_logs_command", {
        id: urlParams.get("id"),
        namespace: urlParams.get("podNamespace"),
        podName: urlParams.get("podName"),
    });
    console.log("Streaming logs end");
};
</script>
