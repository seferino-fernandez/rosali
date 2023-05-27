<template>
    <DataTable :value="jobs" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('jobs.table.no_results') }}</template>
        <template #loading>{{ $t('jobs.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="completions" header="Completions" />
        <Column field="parallelism" header="Parallelism" />
        <Column field="active" header="Active" />
        <Column field="succeeded" header="Succeeded" />
        <Column field="failed" header="Failed" />
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
        const jobs = ref([]);
        const selectedJob = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchJobs();
            } catch (error) {
                console.error("Error fetching jobs data:", error);
            }
        });

        const fetchJobs = async () => {
            const response = await invoke("get_jobs_command", { id: clusterConnectionId.value });
            jobs.value = response.data;
        };

        const onRowClick = (e) => {
            selectedJob.value = e.data;
        };

        return {
            jobs,
            fetchJobs,
            onRowClick,
            selectedJob,
        };
    },
};
</script>
