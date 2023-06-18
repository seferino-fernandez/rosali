<template>
    <DataTable :value="roles" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('roles.table.no_results') }}</template>
        <template #loading>{{ $t('roles.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="namespace" header="Name" />
        <Column field="labels" header="Labels" />
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
        const roles = ref([]);
        const selectedRole = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchRoles();
            } catch (error) {
                console.error("Error fetching roles data:", error);
            }
        });

        const fetchRoles = async () => {
            const response = await invoke("get_roles_command", { id: clusterConnectionId.value });
            roles.value = response.data;
        };

        const onRowClick = (e) => {
            selectedRole.value = e.data;
        };

        return {
            roles,
            fetchRoles,
            onRowClick,
            selectedRole,
        };
    },
};
</script>
