from locust import between, task, FastHttpUser, events, stats
import random

stats.PERCENTILES_TO_CHART = [0.99]

dishes = [
    "A",
    "B",
    "C",
    "D",
    "E",
    "F"
]

@events.init_command_line_parser.add_listener
def _(parser):
    parser.add_argument("--max-orders", type=int, is_required=True, default=5)
    parser.add_argument("--tables", type=int, is_required=True, default=100)

class ApplicationUser(FastHttpUser):
    wait_time = between(0.5, 5)

    @task
    def stuff(self):
        table = random.randint(0, self.environment.parsed_options.tables)
        max_order = random.randint(1, self.environment.parsed_options.max_orders)
        items = self.client.get(f'v1/tables/{table}/items').json()['items']
        if len(items) == 0 or random.randint(0, 1) == 0:
            command = { "items": [{"name": random.choice(dishes)} for _ in range(max_order)] }
            self.client.post(f'v1/tables/{table}', json=command)
        else:
            item = random.choice(items)
            self.client.delete(f'v1/tables/{item["table_id"]}/items/{item["id"]}')

