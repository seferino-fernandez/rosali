export interface Cluster {
  name: string;
  cluster: {
    server: string;
    "certificate-authority": string;
    extensions: Array<any>;
  };
}

export interface User {
  name: string;
  user: {
    "client-certificate": string;
    "client-key": string;
  };
}

export interface Context {
  name: string;
  context: {
    cluster: string;
    user: string;
    namespace: string;
    extensions: Array<any>;
  };
}

export interface KubeconfigData {
  preferences: object;
  clusters: Array<Cluster>;
  users: Array<User>;
  contexts: Array<Context>;
  "current-context": string;
  kind: string;
  apiVersion: string;
}
