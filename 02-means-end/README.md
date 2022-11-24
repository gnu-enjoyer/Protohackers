# Means to an End

```bash
[Thu Nov 24 17:09:24 2022 UTC] [0example.test] NOTE:check starts
[Thu Nov 24 17:09:24 2022 UTC] [0example.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:24 2022 UTC] [0example.test] NOTE:sending example data
[Thu Nov 24 17:09:25 2022 UTC] [0example.test] PASS
[Thu Nov 24 17:09:26 2022 UTC] [1main.test] NOTE:check starts
[Thu Nov 24 17:09:26 2022 UTC] [1main.test] NOTE:4 simultaneous clients
[Thu Nov 24 17:09:27 2022 UTC] [1main.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:27 2022 UTC] [1main.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:27 2022 UTC] [1main.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:27 2022 UTC] [1main.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:31 2022 UTC] [1main.test] PASS
[Thu Nov 24 17:09:32 2022 UTC] [2largedata.test] NOTE:check starts
[Thu Nov 24 17:09:32 2022 UTC] [2largedata.test] NOTE:inserting 200k prices in random order
[Thu Nov 24 17:09:32 2022 UTC] [2largedata.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:35 2022 UTC] [2largedata.test] PASS
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:check starts
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:checking whether bad clients can disrupt good clients
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:sending an incomplete message
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:sending an illegal message type
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] NOTE:disconnecting immediately after sending a query
[Thu Nov 24 17:09:36 2022 UTC] [3badclient.test] PASS
[Thu Nov 24 17:09:37 2022 UTC] [4intoverflow.test] NOTE:check starts
[Thu Nov 24 17:09:37 2022 UTC] [4intoverflow.test] NOTE:testing for integer overflow
[Thu Nov 24 17:09:38 2022 UTC] [4intoverflow.test] NOTE:connected to xxx.xxx.xx.xxx port 45100
[Thu Nov 24 17:09:38 2022 UTC] [4intoverflow.test] PASS
```