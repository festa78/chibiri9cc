kubectl delete svc goapi compilerapi nginx-srv && kubectl delete deployments.apps compilerapi goapi nginx-proxy && kubectl delete configmaps nginx-conf && kubectl apply -R -f k8s
