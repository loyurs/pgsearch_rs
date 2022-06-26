import json

from locust import HttpUser, TaskSet, task
import json


class Testlocust(TaskSet):
    @task(3)
    def test_queryTotalFeedbackByPage(self):
        post_url = "/insert"
        header = {
            "Accept-Encoding": "gzip, deflate",
            "Content-Type": "application/json;charset=UTF-8",
            "Cookie": "",
        }
        mki = {"title":"外交部：世卫组织已向中方澄清，谭德塞未发表“新冠病毒可能来自武汉实验室泄漏”言论","descriptions":"【环球时报-环球网报道 记者张卉】6月22日外交部发言人汪文斌主持例行记者会。有记者提问称，英国《星期日邮报》援引消息人士的话报道称，世卫组织总干事谭德塞近期曾私下向欧洲高官透露，新冠病毒最有可能的来源是武汉实验室泄漏事故。中方对此有何评论？汪文斌对此表示，世卫组织秘书处已就有关报道向中方作出了澄清，强调谭德塞总干事未在公开或私下场合发表任何此类的言论，你所提到的报道内容完全不属实，总干事完全反对报道中的有关内容。汪文斌称，关于实验室泄漏假设问题，中方已多次阐述有关立场。实验室泄漏完全是反华势力出于政治目的炮制的谎言，毫无科学可言。中方已经邀请世卫组织国际专家组赴武汉相关实验室考察，联合研究报告也明确得出了实验室泄漏极不可能的结论。有关媒体匿名放风的形式翻炒实验室泄漏，罔顾事实，用心险恶，也再次证明实验室泄漏完全是有关方面企图抹黑中国，阻碍科学溯源，破坏国际抗疫合作大局的政治操弄。当前国际科学界有越来越多的线索，将病毒源头指向全球范围。美国政府至今未就新冠肺炎疫情最早何时在美国暴发等重要问题给出令人信服的答案，更未就德特里克堡、北卡罗来纳大学实验室高度可疑的活动，回应国际社会的合理关切。如果一定要就实验室问题进行研究，必须先对美国德特里克堡、北卡罗来纳大学等高度可疑的实验室进行核查。有关方面如果真正关心新冠病毒溯源问题，就应当关注美方为何至今都没有正面回应国际社会的质疑，呼吁美方开放有关实验室，让国际社会去调查，以实际行动支持配合新冠病毒溯源研究。"}

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

    os.system("locust -f /root/workspace/mysearch-tools/test/insert_to_pg.py")
