```json
{
    "pcd_name": "matching data",
    "data_size": {
        "room_pcd_size": 2000,
        "cr_pcd_size": 100,
        "avia_pcd_size": 5000,
    },
    "process_times": {
        "edge_process_time": 0.03,
        "send_process_time": 0.07,
        "matching_process_time": 0.2,
    },
    "pcd": [
        {
            "data_name": "room_pcd",
            "raw_data": [0.1, 1.0, 0.03],
            "points_num": 2000,
        },
        {
            "data_name": "cr_pcd_pcd",
            "raw_data": [0.1, 1.0, 0.03],
            "points_num": 1000,
        },
        {
            "data_name": "avia_pcd_pcd",
            "raw_data": [0.1, 1.0, 0.03],
            "points_num": 5000,
        }
    ],
}
```

```json
{
    "pcd_name": "matching_data",
    "process_times": {
        "edge": 0.03,
        "send": 0.07,
        "matching": 0.2
    },
    "pcd": [
        {
            "data_name": "room_pcd",
            "raw_data": [0.1, 1.0, 0.03],
            "points_num": 2000
        },
        {
            "data_name": "cr_pcd",
            "raw_data": [0.1, 1.0, 0.03],
            "points_num": 1000
        },
        {
            "data_name": "avia_pcd",
            "raw_data": [0.1, 1.0, 0.03],
            "points_num": 5000
        }
    ]
}
```