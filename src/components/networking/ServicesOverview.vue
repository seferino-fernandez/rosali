<template>
    <DataTable :value="services" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('services.table.no_results') }}</template>
        <template #loading>{{ $t('services.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="type_" header="Type" />
        <Column field="cluster_ip" header="Cluster IP" />
        <Column field="ports" header="Ports" />
        <Column field="external_ip" header="External IP" />
        <Column field="selector" header="Selector" />
        <Column field="age" header="Age" />
        <Column field="status" header="Status" />
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
        const services = ref([]);
        const selectedService = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchServices();
            } catch (error) {
                console.error("Error fetching services data:", error);
            }
        });

        const fetchServices = async () => {
            const response = await invoke("get_services_command", { id: clusterConnectionId.value });
            services.value = response.data;
        };

        const onRowClick = (e) => {
            selectedService.value = e.data;
        };

        return {
            services,
            fetchServices,
            onRowClick,
            selectedService,
        };
    },
};
</script>
