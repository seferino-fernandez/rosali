<template>
    <div class="pod-overview">
        <DataTable :value="pods" @row-click="onRowClick" dataKey="name">
            <template #header>
                <div class="">
                    <span class="">{{ $t('kubernetes.pod', 2) }}</span>
                    <span class="">({{ pods ? pods.length : 0 }})</span>
                    <Button icon="pi pi-refresh" rounded raised @click="fetchPods" />
                </div>
            </template>
            <Column field="name" :header="$t('kubernetes.meta.name', 1)" />
            <Column field="namespace" :header="$t('kubernetes.namespace', 1)" />
            <Column field="status" :header="$t('kubernetes.meta.status', 1)" />
            <Column field="restarts" :header="$t('kubernetes.meta.restart', 2)" />
            <Column field="age" :header="$t('kubernetes.meta.age')" />
        </DataTable>

        <Sidebar v-model:visible="showDetails" position="right">
            <template #header>
                <h3>{{ selectedPod.name }}</h3>
                <Button :label="$t('kubernetes.log', 2)" icon="pi pi-file" class="p-mr-2" @click="viewLogs" />
                <Divider />
            </template>
        </Sidebar>
    </div>
</template>
  
<script>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import Button from "primevue/button";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Sidebar from 'primevue/sidebar';
import { invoke } from "@tauri-apps/api";
import Divider from 'primevue/divider';
import ScrollPanel from 'primevue/scrollpanel';
import hljs from 'highlight.js/lib/common';
import hljsVuePlugin from "@highlightjs/vue-plugin";

export default {
    components: {
        Button,
        DataTable,
        Column,
        Sidebar,
        Divider,
        ScrollPanel,
        highlightjs: hljsVuePlugin.component,
    },
    setup(props, { emit }) {
        const route = useRoute();
        const pods = ref([]);
        const selectedPod = ref({});
        const clusterConnectionId = ref(route.params.id);
        const showDetails = ref(false);
        const showLogs = ref(false);

        onMounted(async () => {
            try {
                fetchPods();
            } catch (error) {
                console.error("Error fetching pods data:", error);
            }
        });

        const fetchPods = async () => {
            const response = await invoke("get_pods_command", { id: clusterConnectionId.value });
            pods.value = response.data;
        };

        const viewLogs = async (e) => {
            showDetails.value = false;
            emit('view-logs', {
                connectionId: clusterConnectionId.value,
                pod: selectedPod.value,
            });
        };

        const onRowClick = (e) => {
            selectedPod.value = e.data;
            showDetails.value = true;
        };

        return {
            pods,
            fetchPods,
            onRowClick,
            viewLogs,
            showDetails,
            showLogs,
            selectedPod,
            emit,
        };
    },
};
</script>
<style scoped></style>