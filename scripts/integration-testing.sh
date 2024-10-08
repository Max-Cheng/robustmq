#!/bin/sh
start_placement_center(){
    nohup cargo run --package cmd --bin $placement_center_process_name -- --conf=tests/config/$placement_center_process_name.toml >/dev/null 2>&1 &
    sleep 3
    while ! ps aux | grep -v grep | grep "$placement_center_process_name" > /dev/null; do
        echo "Process $placement_center_process_name has not started yet, wait 1s...."
        sleep 1  
    done
    echo "Process $placement_center_process_name starts successfully and starts running the test case"
}

stop_placement_center(){
    pc_no=`ps aux | grep -v grep | grep "$placement_center_process_name" | awk '{print $2}'`
    echo "placement center num: $pc_no"
    kill $pc_no
    sleep 3

    while ps aux | grep -v grep | grep "$placement_center_process_name" > /dev/null; do
        echo "”Process $placement_center_process_name stopped successfully"
        sleep 1  
    done
}

start_mqtt_broker_1(){
    nohup cargo run --package cmd --bin $mqtt_server_process_name -- --conf=tests/config/$mqtt_server_process_name-1.toml >/dev/null 2>&1 &
    sleep 3
    while ! ps aux | grep -v grep | grep "$mqtt_server_process_name-1" > /dev/null; do
        echo "Process $mqtt_server_process_name-1 has not started yet, wait 1s...."
        sleep 1  
    done
    echo "Process $mqtt_server_process_name-1 starts successfully and starts running the test case"
}

stop_mqtt_broker_1(){
    mqtt_no=`ps aux | grep -v grep | grep "$mqtt_server_process_name-1" | awk '{print $2}'`
    echo "mqtt server num: $mqtt_no"
    kill $mqtt_no
    sleep 3

    while ps aux | grep -v grep | grep "$mqtt_server_process_name-1" > /dev/null; do
        echo "”Process $mqtt_server_process_name-1 stopped successfully"
        sleep 1  
    done
}

stop_server(){
    # Stop mqtt-broker
    sleep 1
    stop_mqtt_broker_1

    # Stop placement-center
    sleep 1
    stop_placement_center
}

rm -rf /tmp/robust/tests/mqtt-broker
rm -rf /tmp/robust/tests/placement-center

placement_center_process_name="placement-center"
mqtt_server_process_name="mqtt-server"

# Start placement-center
start_placement_center

# Start mqtt-broker
start_mqtt_broker_1

sleep 3
# Run Cargo Test
cargo nextest run

if [ $? -ne 0 ]; then
    echo "Test case failed to run"
    stop_server
    exit 1
else
    echo "Test case runs successfully"
    stop_server
fi

