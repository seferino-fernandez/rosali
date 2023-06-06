<template>
    <DataTable :value="nodes" @row-click="onRowClick" :paginator="true" :rows="10">
        <template #empty>{{ $t('nodes.table.no_results') }}</template>
        <template #loading>{{ $t('nodes.table.loading') }}</template>
        <Column field="name" header="Name" />
        <Column field="age" header="Age" />
        <Column field="taints" header="Taints" />
        <Column field="version" header="Version" />
        <Column field="conditions" header="Conditions">
            <template #body="slotProps">
                <div v-for="(condition, index) in slotProps.data.conditions" :key="index">
                    <span v-tooltip.top="condition.message" :class="getConditionClass(condition)">
                        {{ condition.type_ }}
                        <br />
                    </span>
                </div>
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

export default {
    components: {
        DataTable,
        Column,
    },
    setup() {
        const route = useRoute();
        const nodes = ref([]);
        const selectedNode = ref({});
        const clusterConnectionId = ref(route.params.id);

        onMounted(async () => {
            try {
                fetchNodes();
            } catch (error) {
                console.error("Error fetching nodes data:", error);
            }
        });

        const fetchNodes = async () => {
            const response = await invoke("get_nodes_command", { id: clusterConnectionId.value });
            nodes.value = response.data;
        };

        const onRowClick = (e) => {
            selectedNode.value = e.data;
        };

        const getConditionClass = (condition) => {
            if (condition.type_ === 'Ready') {
                return condition.status === 'True' ? 'text-green' : 'text-red';
            } else {
                return condition.status === 'True' ? 'text-red' : 'text-green';
            }
        };

        return {
            nodes,
            fetchNodes,
            onRowClick,
            selectedNode,
            getConditionClass
        };
    },
};
</script>

<style scoped>
.text-green {
    color: green;
}

.text-red {
    color: red;
}
</style>
