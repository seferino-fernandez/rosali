import { createRouter, createMemoryHistory } from "vue-router";
import KubeconfigContexts from "@/components/KubeconfigContexts.vue";
import ClusterOverview from "@/components/ClusterOverview.vue";
import CronJobsOverview from "@/components/workloads/CronJobsOverview.vue";
import PodsOverview from "@/components/PodsOverview.vue";
import DeploymentsOverview from "@/components/DeploymentsOverview.vue";
import DaemonSetsOverview from "@/components/DaemonSetsOverview.vue";
import JobsOverview from "@/components/workloads/JobsOverview.vue";
import ReplicaSetsOverview from "@/components/ReplicaSetsOverview.vue";
import ReplicationControllersOverview from "@/components/workloads/ReplicationControllersOverview.vue";
import ServicesOverview from "@/components/services/ServicesOverview.vue";
import StatefulSetsOverview from "@/components/StatefulSetsOverview.vue";

const routes = [
  {
    path: "/",
    component: KubeconfigContexts,
    name: "KubeconfigContexts",
  },
  {
    path: "/cluster-overview/:id",
    name: "ClusterOverview",
    component: ClusterOverview,
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
    path: "/jobs-overview/:id",
    name: "JobsOverview",
    component: JobsOverview,
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
    path: "/services-overview/:id",
    name: "ServicesOverview",
    component: ServicesOverview,
  },
  {
    path: "/statefulsets-overview/:id",
    name: "StatefulSetsOverview",
    component: StatefulSetsOverview,
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

// Add this beforeEach hook
router.beforeEach((to, from, next) => {
  if ((to.name !== "KubeconfigContexts") && !to.params.id) {
    next({ name: "KubeconfigContexts" });
  } else {
    next();
  }
});

export default router;
