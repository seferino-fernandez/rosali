<template>
    <DataTable :value="networkPolicies" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('networkpolicies.table.no_results') }}</template>
        <template #loading>{{ $t('networkpolicies.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Namespace" />
        <Column field="policy_types" header="Policy Types" />
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
        const networkPolicies = ref([]);
        const selectedNetworkPolicy = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchNetworkPolicies();
            } catch (error) {
                console.error("Error fetching network policies data:", error);
            }
        });

        const fetchNetworkPolicies = async () => {
            const response = await invoke("get_network_policies_command", { id: clusterConnectionId.value });
            networkPolicies.value = response.data;
        };

        const onRowClick = (e) => {
            selectedNetworkPolicy.value = e.data;
        };

        return {
            networkPolicies,
            fetchNetworkPolicies,
            onRowClick,
            selectedNetworkPolicy,
        };
    },
};
</script>
