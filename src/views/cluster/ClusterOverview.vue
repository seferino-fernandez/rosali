<template>
    <div class="overflow-auto">
        <div class="flex flex-inline justify-content-center p-4">
            <div class="flex-column">
                <div class="text-base text-color text-center font-bold">{{ $t('kubernetes.deployment', 2) }}</div>
                <Chart class="w-15rem h-15rem" type="doughnut" :data="deploymentChartData" />
            </div>
            <div class="flex-column">
                <div class="text-base text-color text-center font-bold">{{ $t('kubernetes.pod', 2) }}</div>
                <Chart class="w-15rem h-15rem" type="doughnut" :data="podChartData" />
            </div>
            <div class="flex-column">
                <div class="text-base text-color text-center font-bold">{{ $t('kubernetes.replica_set', 2) }}</div>
                <Chart class="w-15rem h-15rem" type="doughnut" :data="replicaSetChartData" />
            </div>
        </div>
        <DataTable scrollable class="p-datatable-sm" :value="recentEvents">
            <template #header>
                <div class="">
                    <span class="text-base text-color font-bold">{{ $t('cluster_overview.recent_events') }}</span>
                    <span class="font-light"> ({{ recentEvents ? recentEvents.length : 0 }})</span>
                </div>
            </template>
            <Column field="warning_or_regular" :header="$t('events.type')">
                <template #body="slotProps">
                    <span :class="{ 'warning-indicator': slotProps.data.warning_or_regular === 'Warning' }">{{
                        slotProps.data.warning_or_regular }}</span>
                </template>
            </Column>
            <Column field="name" :header="$t('kubernetes.meta.name', 1)"></Column>
            <Column field="namespace" :header="$t('kubernetes.namespace', 1)"></Column>
            <Column field="reason" :header="$t('events.reason')"></Column>
            <Column field="message" :header="$t('events.message')"></Column>
            <Column field="source" :header="$t('events.source')"></Column>
            <Column field="count" :header="$t('events.count')"></Column>
            <Column field="first_time" :header="$t('events.first_time')"></Column>
            <Column field="last_time" :header="$t('events.last_time')"></Column>
            <template #empty>{{ $t('events.table.no_results') }}</template>
            <template #loading>{{ $t('events.table.loading') }}</template>
        </DataTable>
    </div>
</template>
  
<script>
import { ref, onMounted, watch, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useI18n } from 'vue-i18n'
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
        const { t } = useI18n({ useScope: 'global' })

        const clusterConnectionId = ref(route.params.id);
        const clusterNamespace = ref(route.query.namespace);
        const recentEvents = ref([]);

        const deploymentChartData = ref({});
        const podChartData = ref({});
        const replicaSetChartData = ref({});

        onMounted(async () => {
            fetchWorkloads();
            fetchRecentEvents();
        });

        const fetchWorkloads = async () => {
            try {
                const documentStyle = getComputedStyle(document.body);

                const response = await invoke("get_context_overview", { id: clusterConnectionId.value, namespace: clusterNamespace.value });
                let clusterStatusOverview = response.data;

                deploymentChartData.value = {
                    labels: [t('kubernetes.meta.status.running', clusterStatusOverview.deployments.running), t('kubernetes.meta.status.failed', clusterStatusOverview.deployments.failed)],
                    datasets: [
                        {
                            data: [clusterStatusOverview.deployments.running, clusterStatusOverview.deployments.failed],
                            backgroundColor: [documentStyle.getPropertyValue('--blue-300'), documentStyle.getPropertyValue('--red-300')],
                        },
                    ],
                };

                podChartData.value = {
                    labels: [t('kubernetes.meta.status.running', clusterStatusOverview.pods.running,), t('kubernetes.meta.status.failed', clusterStatusOverview.pods.failed)],
                    datasets: [
                        {
                            data: [clusterStatusOverview.pods.running, clusterStatusOverview.pods.failed],
                            backgroundColor: [documentStyle.getPropertyValue('--blue-300'), documentStyle.getPropertyValue('--red-300')],
                        },
                    ],
                };

                replicaSetChartData.value = {
                    labels: [t('kubernetes.meta.status.running', clusterStatusOverview.replicas.running), t('kubernetes.meta.status.failed', clusterStatusOverview.replicas.failed)],
                    datasets: [
                        {
                            data: [clusterStatusOverview.replicas.running, clusterStatusOverview.replicas.failed],
                            backgroundColor: [documentStyle.getPropertyValue('--blue-300'), documentStyle.getPropertyValue('--red-300')],
                        },
                    ],
                };
            } catch (error) {
                console.error("Error fetching workloads overview data:", error);
            }
        };

        const fetchRecentEvents = async () => {
            try {
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
            } catch (error) {
                console.error("Error fetching recent events data:", error);
            }
        };

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
    color: var(--red-300);
    font-weight: bold;
}
</style>