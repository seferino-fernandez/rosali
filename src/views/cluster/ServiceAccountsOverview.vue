<template>
    <DataTable :value="serviceAccounts" @row-click="onRowClick">
        <template #empty>{{ $t('serviceaccounts.table.no_results') }}</template>
        <template #loading>{{ $t('serviceaccounts.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
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
        const serviceAccounts = ref([]);
        const selectedServiceAccount = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchServiceAccounts();
            } catch (error) {
                console.error("Error fetching service accounts data:", error);
            }
        });

        const fetchServiceAccounts = async () => {
            const response = await invoke("get_service_accounts_command", { id: clusterConnectionId.value });
            serviceAccounts.value = response.data;
        };

        const onRowClick = (e) => {
            selectedServiceAccount.value = e.data;
        };

        return {
            serviceAccounts,
            fetchServiceAccounts,
            onRowClick,
            selectedServiceAccount,
        };
    },
};
</script>
