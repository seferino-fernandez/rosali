<template>
    <div class="layout-sidebar">
        <PanelMenu class=".sidebar-items" :model="items" @item-click="navigateTo" />
    </div>
</template>
  
<script>
import { computed, onMounted, watch, ref } from "vue";
import { useRouter } from "vue-router";
import PanelMenu from "primevue/panelmenu";
import ScrollPanel from "primevue/scrollpanel";


export default {
    components: {
        PanelMenu,
        ScrollPanel,
    },
    setup() {
        const router = useRouter();

        const currentId = ref("");
        const items = ref([]);

        const homePage = { name: "KubeconfigContexts", params: {} };

        watch(
            () => router.currentRoute.value,
            () => {
                currentId.value = router.currentRoute.value.name !== "KubeconfigContexts" ? router.currentRoute.value.params.id : "";
                items.value = [
                    {
                        label: "Workloads",
                        items: [
                            { label: "Pods", to: currentId.value ? { name: "PodsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Deployments", to: currentId.value ? { name: "DeploymentsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Jobs", to: currentId.value ? { name: "JobsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Cron Jobs", to: currentId.value ? { name: "CronJobsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Daemon Sets", to: currentId.value ? { name: "DaemonSetsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Replica Sets", to: currentId.value ? { name: "ReplicaSetsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Stateful Sets", to: currentId.value ? { name: "StatefulSetsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Replication Controllers", to: currentId.value ? { name: "ReplicationControllersOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        label: "Networking",
                        items: [
                            { label: "Services", to: currentId.value ? { name: "ServicesOverview", params: { id: currentId.value } } : homePage },
                            { label: "Endpoints" },
                            { label: "Ingresses", to: currentId.value ? { name: "IngressesOverview", params: { id: currentId.value } } : homePage },
                            { label: "Ingress Classes", to: currentId.value ? { name: "IngressClassesOverview", params: { id: currentId.value } } : homePage },
                            { label: "Network Polices" },
                        ],
                    },
                    {
                        label: "Config and Storage",
                        items: [
                            { label: "Config Maps", to: currentId.value ? { name: "ConfigMapsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Secrets", to: currentId.value ? { name: "SecretsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Persistent Volumn Claims", to: currentId.value ? { name: "PersistentVolumnClaimsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Storage Classes", to: currentId.value ? { name: "StorageClassesOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                    {
                        label: "Cluster",
                        items: [
                            { label: "Namespaces" },
                            { label: "Nodes" },
                            { label: "Service Accounts" },
                        ],
                    },
                    {
                        label: "Access Control",
                        items: [
                            { label: "Cluster Roles", to: currentId.value ? { name: "ClusterRolesOverview", params: { id: currentId.value } } : homePage },
                            { label: "Cluster Role Bindings", to: currentId.value ? { name: "ClusterRoleBindingsOverview", params: { id: currentId.value } } : homePage },
                            { label: "Roles", to: currentId.value ? { name: "RolesOverview", params: { id: currentId.value } } : homePage },
                            { label: "Role Bindings", to: currentId.value ? { name: "RoleBindingsOverview", params: { id: currentId.value } } : homePage },
                        ],
                    },
                ];
            },
            { immediate: true }
        );

        function navigateTo(item) {
            console.error(item);
            if (item.to) {
                router.push(item.to);
            }
        }
        return {
            visible: false,
            items: items,
            navigateTo,
        };
    },
};
</script>
  
<style scoped>
.layout-sidebar {
    z-index: 998;
    user-select: none;
    background-color: var(--surface-overlay);
    box-shadow: 0px 3px 5px rgba(0, 0, 0, 0.02), 0px 0px 2px rgba(0, 0, 0, 0.05), 0px 1px 4px rgba(0, 0, 0, 0.08);
}
</style>