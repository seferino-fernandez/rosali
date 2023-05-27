<template>
    <div class="pod-overview">
        <Button @click="fetchPods" rounded icon="pi pi-refresh" />

        <DataTable :value="pods" @row-click="onRowClick" dataKey="id" :paginator="true" :rows="10">
            <Column field="name" header="Name" />
            <Column field="namespace" header="Namespace" />
            <Column field="status" header="Status" />
            <Column field="restarts" header="Restarts" />
            <Column field="age" header="Age" />
        </DataTable>

        <Sidebar v-model:visible="showDetails" position="right">
            <template #header>
                <h3>{{ selectedPod.name }}</h3>
                <Button label="Logs" icon="pi pi-file" class="p-mr-2" @click="viewLogs" />
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
import { invoke, event } from "@tauri-apps/api";
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
            console.log(clusterConnectionId.value);
            const response = await invoke("get_pods_command", { id: clusterConnectionId.value });
            console.log(response);
            pods.value = response.data;
        };

        const viewLogs = async () => {
            console.log('viewLogs called');
            console.log('Emitting view-logs event:', {
                connectionId: clusterConnectionId.value,
                pod: selectedPod.value,
            });
            showDetails.value = false;
            emit('view-logs', {
                connectionId: clusterConnectionId.value,
                pod: selectedPod.value,
            });
        };

        const onRowClick = (e) => {
            console.log(e);
            selectedPod.value = e.data;
            showDetails.value = true;
        };

        const execPod = (rowData) => {
        };

        const deletePod = (rowData) => {
        };

        return {
            pods,
            fetchPods,
            onRowClick,
            viewLogs,
            execPod,
            deletePod,
            showDetails,
            showLogs,
            selectedPod,
            emit,
        };
    },
};
</script>
<style scoped>
.pod-overview {
    padding: 1rem;
}
</style>