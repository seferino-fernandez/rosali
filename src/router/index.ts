import { createRouter, createMemoryHistory } from "vue-router";
import KubeconfigContexts from "@/components/KubeconfigContexts.vue";
import ClusterOverview from "@/components/ClusterOverview.vue";
import ClusterRolesOverview from "@/components/access_control/ClusterRolesOverview.vue";
import ClusterRoleBindingsOverview from "@/components/access_control/ClusterRoleBindingsOverview.vue";
import ConfigMapsOverview from "@/components/config_storage/ConfigMapsOverview.vue";
import CronJobsOverview from "@/components/workloads/CronJobsOverview.vue";
import PodsOverview from "@/components/PodsOverview.vue";
import DeploymentsOverview from "@/components/DeploymentsOverview.vue";
import DaemonSetsOverview from "@/components/DaemonSetsOverview.vue";
import IngressesOverview from "@/components/networking/IngressesOverview.vue";
import IngressClassesOverview from "@/components/networking/IngressClassesOverview.vue";
import JobsOverview from "@/components/workloads/JobsOverview.vue";
import NamespacesOverview from "@/components/cluster/NamespacesOverview.vue";
import NodesOverview from "@/components/cluster/NodesOverview.vue";
import PersistentVolumnClaimsOverview from "@/components/config_storage/PersistentVolumnClaimsOverview.vue";
import ReplicaSetsOverview from "@/components/ReplicaSetsOverview.vue";
import ReplicationControllersOverview from "@/components/workloads/ReplicationControllersOverview.vue";
import RolesOverview from "@/components/access_control/RolesOverview.vue";
import RoleBindingsOverview from "@/components/access_control/RoleBindingsOverview.vue";
import SecretsOverview from "@/components/config_storage/SecretsOverview.vue";
import ServiceAccountsOverview from "@/components/cluster/ServiceAccountsOverview.vue";
import ServicesOverview from "@/components/networking/ServicesOverview.vue";
import StatefulSetsOverview from "@/components/StatefulSetsOverview.vue";
import StorageClassesOverview from "@/components/config_storage/StorageClassesOverview.vue";

const routes = [
  {
    path: "/",
    component: KubeconfigContexts,
    name: "KubeconfigContexts",
  },
  {
    path: "/configmaps-overview/:id",
    name: "ConfigMapsOverview",
    component: ConfigMapsOverview,
  },
  {
    path: "/cluster-overview/:id",
    name: "ClusterOverview",
    component: ClusterOverview,
  },
  {
    path: "/clusterroles-overview/:id",
    name: "ClusterRolesOverview",
    component: ClusterRolesOverview,
  },
  {
    path: "/clusterrolebindings-overview/:id",
    name: "ClusterRoleBindingsOverview",
    component: ClusterRoleBindingsOverview,
  },
  {
    path: "/cronjobs-overview/:id",
    name: "CronJobsOverview",
    component: CronJobsOverview,
  },
  {
    path: "/pods-overview/:id",
    name: "PodsOverview",
    component: PodsOverview,
  },
  {
    path: "/deployments-overview/:id",
    name: "DeploymentsOverview",
    component: DeploymentsOverview,
  },
  {
    path: "/daemonsets-overview/:id",
    name: "DaemonSetsOverview",
    component: DaemonSetsOverview,
  },
  {
    path: "/ingresses-overview/:id",
    name: "IngressesOverview",
    component: IngressesOverview,
  },
  {
    path: "/ingress-classes-overview/:id",
    name: "IngressClassesOverview",
    component: IngressClassesOverview,
  },
  {
    path: "/jobs-overview/:id",
    name: "JobsOverview",
    component: JobsOverview,
  },
  {
    path: "/namespaces-overview/:id",
    name: "NamespacesOverview",
    component: NamespacesOverview,
  },
  {
    path: "/nodes-overview/:id",
    name: "NodesOverview",
    component: NodesOverview,
  },
  {
    path: "/persistent-volumn-claims-overview/:id",
    name: "PersistentVolumnClaimsOverview",
    component: PersistentVolumnClaimsOverview,
  },
  {
    path: "/replicasets-overview/:id",
    name: "ReplicaSetsOverview",
    component: ReplicaSetsOverview,
  },
  {
    path: "/replicationcontrollers-overview/:id",
    name: "ReplicationControllersOverview",
    component: ReplicationControllersOverview,
  },
  {
    path: "/roles-overview/:id",
    name: "RolesOverview",
    component: RolesOverview,
  },
  {
    path: "/rolebindings-overview/:id",
    name: "RoleBindingsOverview",
    component: RoleBindingsOverview,
  },
  {
    path: "/secrets-overview/:id",
    name: "SecretsOverview",
    component: SecretsOverview,
  },
  {
    path: "/serviceaccounts-overview/:id",
    name: "ServiceAccountsOverview",
    component: ServiceAccountsOverview,
  },
  {
    path: "/services-overview/:id",
    name: "ServicesOverview",
    component: ServicesOverview,
  },
  {
    path: "/statefulsets-overview/:id",
    name: "StatefulSetsOverview",
    component: StatefulSetsOverview,
  },
  {
    path: "/storage-classes-overview/:id",
    name: "StorageClassesOverview",
    component: StorageClassesOverview,
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

// Add this beforeEach hook
router.beforeEach((to, from, next) => {
  if (to.name !== "KubeconfigContexts" && !to.params.id) {
    next({ name: "KubeconfigContexts" });
  } else {
    next();
  }
});

export default router;
