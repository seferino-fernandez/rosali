<template>
    <div class="overflow-auto">
        <div class="flex flex-inline justify-content-center p-4">
            <div class="flex-column">
                <div class="text-base text-color text-center font-bold">{{ $t('cluster_overview.deployments') }}</div>
                <Chart class="w-15rem h-15rem" type="doughnut" :data="deploymentChartData" />
            </div>
            <div class="flex-column">
                <div class="text-base text-color text-center font-bold">{{ $t('cluster_overview.pods') }}</div>
                <Chart class="w-15rem h-15rem" type="doughnut" :data="podChartData" />
            </div>
            <div class="flex-column">
                <div class="text-base text-color text-center font-bold">{{ $t('cluster_overview.replica_sets') }}</div>
                <Chart class="w-15rem h-15rem" type="doughnut" :data="replicaSetChartData" />
            </div>
        </div>
        <DataTable scrollable class="p-datatable-sm" :value="recentEvents">
            <template #header>
                <div class="">
                    <span class="text-base text-color font-bold">{{ $t('cluster_overview.recent_events', 2) }}</span>
                    <span class="font-light"> ({{ recentEvents ? recentEvents.length : 0 }})</span>
                </div>
            </template>
            <Column field="warning_or_regular" header="Type">
                <template #body="slotProps">
                    <span :class="{ 'warning-indicator': slotProps.data.warning_or_regular === 'Warning' }">{{
                        slotProps.data.warning_or_regular }}</span>
                </template>
            </Column>
            <Column field="name" header="Name"></Column>
            <Column field="namespace" header="Namespace"></Column>
            <Column field="reason" header="Reason"></Column>
            <Column field="message" header="Message"></Column>
            <Column field="source" header="Source"></Column>
            <Column field="count" header="Count"></Column>
            <Column field="first_time" header="First Time"></Column>
            <Column field="last_time" header="Last Time"></Column>
            <template #empty>{{ $t('events.table.no_results') }}</template>
            <template #loading>{{ $t('events.table.loading') }}</template>
        </DataTable>
    </div>
</template>
  
<script>
import { ref, onMounted, watch, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import Chart from 'primevue/chart';
import Divider from 'primevue/divider';
import DataTable from 'primevue/datatable';
import Column from "primevue/column";
import { invoke } from "@tauri-apps/api";

export default {
    components: {
        Chart,
        Divider,
        DataTable,
        Column
    },
    setup() {
        const router = useRouter();
        const route = useRoute();
        const clusterConnectionId = ref(route.params.id);
        const clusterNamespace = ref(route.query.namespace);
        const recentEvents = ref([]);

        const deploymentChartData = ref({});
        const podChartData = ref({});
        const replicaSetChartData = ref({});

        onMounted(async () => {
            try {
                const response = await invoke("get_context_overview", { id: clusterConnectionId.value, namespace: clusterNamespace.value });
                let clusterStatusOverview = response.data;
                const recentEventsResponse = await invoke("get_recent_events_command", { id: clusterConnectionId.value });
                if (recentEventsResponse == null || recentEventsResponse.data == null) {
                    return;
                }
                let unformattedEvents = recentEventsResponse.data;
                recentEvents.value = unformattedEvents.map(event => {
                    return {
                        ...event,
                        first_time: new Date(event.first_time).toLocaleString(),
                        last_time: new Date(event.last_time).toLocaleString(),
                    }
                });

                deploymentChartData.value = {
                    labels: ["Running", "Failed"],
                    datasets: [
                        {
                            data: [clusterStatusOverview.deployments.running, clusterStatusOverview.deployments.failed],
                            backgroundColor: ["#36A2EB", "#FF6384"],
                        },
                    ],
                };

                podChartData.value = {
                    labels: ["Running", "Failed"],
                    datasets: [
                        {
                            data: [clusterStatusOverview.pods.running, clusterStatusOverview.pods.failed],
                            backgroundColor: ["#36A2EB", "#FF6384"],
                        },
                    ],
                };

                replicaSetChartData.value = {
                    labels: ["Running", "Failed"],
                    datasets: [
                        {
                            data: [clusterStatusOverview.replicas.running, clusterStatusOverview.replicas.failed],
                            backgroundColor: ["#36A2EB", "#FF6384"],
                        },
                    ],
                };

            } catch (error) {
                console.error("Error fetching context overview data:", error);
            }
        });

        return {
            clusterConnectionId,
            clusterNamespace,
            deploymentChartData,
            podChartData,
            replicaSetChartData,
            recentEvents,
        };
    },
};
</script>
  
<style>
.warning-indicator {
    color: #FF6384;
    font-weight: bold;
}
</style>