<template>
    <DataTable :value="cronJobs" @row-click="onRowClick">
        <template #empty>{{ $t('cronjobs.table.no_results') }}</template>
        <template #loading>{{ $t('cronjobs.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="schedule" header="Schedule" />
        <Column field="last_schedule" header="Last Schedule" />
        <Column field="suspend" header="Suspended" />
        <Column field="active" header="Active" />
        <Column field="age" header="Age" />
    </DataTable>
</template>

<script>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';

export default {
    components: {
        DataTable,
        Column,
    },
    setup() {
        const route = useRoute();
        const cronJobs = ref([]);
        const selectedCronJob = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchCronJobs();
            } catch (error) {
                console.error("Error fetching cronjobs data:", error);
            }
        });

        const fetchCronJobs = async () => {
            const response = await invoke("get_cronjobs_command", { id: clusterConnectionId.value });
            cronJobs.value = response.data;
        };

        const onRowClick = (e) => {
            selectedCronJob.value = e.data;
        };

        return {
            cronJobs,
            fetchCronJobs,
            onRowClick,
            selectedCronJob,
        };
    },
};
</script>
