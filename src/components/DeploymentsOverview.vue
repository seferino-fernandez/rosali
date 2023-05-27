<template>
    <DataTable :value="deployments" @row-click="onRowClick" :paginator="true" :rows="10">
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="status.replicas" header="Replicas" />
        <Column field="status.updated_replicas" header="Updated Replicas" />
        <Column field="status.ready_replicas" header="Ready Replicas" />
        <Column field="status.available_replicas" header="Available Replicas" />
        <Column>
            <template #body="slotProps">
                <Button @click="restartDeployment(slotProps.data)" label="Restart"></Button>
                <Button @click="deleteDeployment(slotProps.data)" label="Delete"></Button>
            </template>
        </Column>
    </DataTable>
</template>

<script>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Button from 'primevue/button';

export default {
    components: {
        DataTable,
        Column,
        Button,
    },
    setup(props, { emit }) {
        const route = useRoute();
        const deployments = ref([]);
        const selectedDeployment = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchDeployments();
            } catch (error) {
                console.error("Error fetching deployments data:", error);
            }
        });

        const fetchDeployments = async () => {
            console.log(clusterConnectionId.value);
            const response = await invoke("get_deployments_command", { id: clusterConnectionId.value });
            console.log(response);
            deployments.value = response.data;
        };

        const onRowClick = (e) => {
            console.log(e);
            selectedDeployment.value = e.data;
        };

        const restartDeployment = async (rowData) => {
            const response = await invoke("restart_deployment_command", { id: clusterConnectionId.value, name: rowData.name, namespace: rowData.namespace });
        };

        const deleteDeployment = (rowData) => {
            // delete operation here
        };

        return {
            deployments,
            fetchDeployments,
            onRowClick,
            restartDeployment,
            deleteDeployment,
            selectedDeployment,
        };
    },
};
</script>