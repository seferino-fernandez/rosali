<template>
    <DataTable :value="ingressClasses" @row-click="onRowClick">
        <template #empty>{{ $t('ingress_classes.table.no_results') }}</template>
        <template #loading>{{ $t('ingress_classes.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="controller" header="Controller" />
        <Column field="api_group" header="API Group" />
        <Column field="scope" header="Scope" />
        <Column field="kind" header="Kind" />
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
        const ingressClasses = ref([]);
        const selectedIngressClass = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchIngressClasses();
            } catch (error) {
                console.error("Error fetching ingress classes data:", error);
            }
        });

        const fetchIngressClasses = async () => {
            const response = await invoke("get_ingress_classes_command", { id: clusterConnectionId.value });
            ingressClasses.value = response.data;
        };

        const onRowClick = (e) => {
            selectedIngressClass.value = e.data;
        };

        return {
            ingressClasses,
            fetchIngressClasses,
            onRowClick,
            selectedIngressClass,
        };
    },
};
</script>
