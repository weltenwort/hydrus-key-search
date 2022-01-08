import json
import subprocess

def try_key(params):
    (key, driver, meter_id, telegram) = params
    args = [
        "/wmbusmeters/wmbusmeters",
        "--silent",
        "--format=json",
        telegram,
        "hydrus",
        driver,
        meter_id,
        str(key),
    ]
    result = subprocess.run(args, stdout=subprocess.PIPE)
    if len(result.stdout) > 0:
        try:
            values = json.loads(result.stdout)
            if (values.get("total_m3", 0) != 0) or (values.get("remaining_battery_life_y", 0) != 0):
                return ("values", key, result.stdout)
            else:
                return ("decode", key, result.stdout)
        except json.JSONDecodeError:
            return ("nojson", key, result.stdout)
    else:
        return ("empty", key)
