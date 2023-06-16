<template>
    <div class="cluster-overview-container flex flex-column md:flex-row md:justify-content-between row-gap-3">
        <div class="charts">
            <div class="chart-container">
                <h3>{{ $t('cluster_overview.deployments') }}</h3>
                <Chart type="doughnut" :data="deploymentChartData" />
            </div>
            <div class="chart-container">
                <h3>{{ $t('cluster_overview.pods') }}</h3>
                <Chart type="doughnut" :data="podChartData" />
            </div>
            <div class="chart-container">
                <h3>{{ $t('cluster_overview.replica_sets') }}</h3>
                <Chart type="doughnut" :data="replicaSetChartData" />
            </div>
        </div>
        <h3>{{ $t('cluster_overview.recent_events') }}</h3>
        <DataTable :value="recentEvents" scrollable scrollHeight="flex" class="p-datatable-sm">
            <template #empty>{{ $t('events.table.no_results') }}</template>
            <template #loading>{{ $t('events.table.loading') }}</template>
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
        const recentEvents = ref([]);

        const deploymentChartData = ref({});
        const podChartData = ref({});
        const replicaSetChartData = ref({});

        const connectionId = ref("");

        onMounted(async () => {
            try {
                const response = await invoke("get_context_overview", { id: clusterConnectionId.value });
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
            deploymentChartData,
            podChartData,
            replicaSetChartData,
            recentEvents,
            connectionId
        };
    },
};
</script>
  
<style>
.cluster-overview-container {
    padding: 1rem;
}

.charts {
    display: flex;
    justify-content: space-evenly;
}

.chart-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.warning-indicator {
    color: #FF6384;
    font-weight: bold;
}
</style>