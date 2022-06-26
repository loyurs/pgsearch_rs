import json

from locust import HttpUser, TaskSet, task
import json

mki = {
    "title": "你好，中国",
    "descriptions": "Tokio 任务是一个异步绿色线程。它们是通过将 async块传递给来创建的tokio::spawn。该tokio::spawn函数返回 a JoinHandle，调用者可以使用它与生成的任务进行交互。该 async块可能有一个返回值。.await调用者可以使用on获取返回值JoinHandle。",
}
# 测试插入
class Testlocust(TaskSet):
    @task(3)
    def test_queryTotalFeedbackByPage(self):
        post_url = ""
        header = {
            "Accept-Encoding": "gzip, deflate",
            "Content-Type": "application/json;charset=UTF-8",
            "Cookie": "",
        }
        mki = {
            "title": "Nihao",
            "descriptions": "Tokio 任务是一个异步绿色线程。它们是通过将 async块传递给来创建的tokio::spawn。该tokio::spawn函数返回 a JoinHandle，调用者可以使用它与生成的任务进行交互。该 async块可能有一个返回值。.await调用者可以使用on获取返回值JoinHandle。",
        }

        s = json.dumps(mki)
        kkk = json.loads(s)
        r = self.client.post(url=post_url, headers=header, json=kkk)
        print(r)


class WebsiteUser(HttpUser):
    tasks = [Testlocust]
    min_wait = 500
    max_wait = 5000


##下面这些可以不用写
if __name__ == "__main__":
    import os

    os.system("locust -f py_test_model.py")
