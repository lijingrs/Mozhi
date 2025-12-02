##  前言
AI时代，dify、n8n、langchain、CriewAI、扣子、飞书 等已经积累了很庞大的用户群体，我认为超级个体再做同类产品已经没有任何意义。通过集成方式海纳百川，不局限于Agent的形式、技术栈、框架，核心关注如何实现Agent驱动的产品目标，发掘产品的价值，形成独有的护城河。

基于企业真实案例，目前没有找到一个能满足我所在企业千人千面Agent的开源生态，目前我在做企业级AIOS，做真正意义上AI驱动的软硬结合的（当前为教育硬件）标杆产品。

AI时代的业务系统，API是入口，Agent是消费者，GUI只是Fallback.业务系统必须满足被AI调用和为AI设计。

AI Native:
- 内建RAG/GRAPH 能力
- 内建WorkFlow/Orchestration
- 内建事件流（Event Bus）
- 内建权限/审计/记录/追踪系统
- 内建工具注册中心

GUI：
- 监督Agent
- 批准/修正Agent的行为/动作
- 查看Agent的思维过程
- 配置Agent的业务逻辑

业务流：
数据层（RAG、GRAPH、事件流）--> 提供给Agent的API --> 低代码构建业务功能 --> GUI
## 项目动态
2025-11-24 增加用户登录功能，满足当前阶段调式接口需要用户凭证的需求。
## **项目介绍**

aimos是一款集智能体管理、开发、测试与应用于一体的智能平台，致力于打造面向AIOS的Agent应用生态。

> react版： https://github.com/aios-rs/aimos-react
> server:  https://github.com/aios-rs/aimos-server

> 项目正在重构，请勿从源码构建当前项目，查看release页面选择最新的安装包体验！

**基于moly二次开发。**

当前正在重构为新一代AI友好的框架

## 演示[视频预览 - 请点击观看]

[![视频预览 - 请点击观看](https://img.youtube.com/vi/dqhfPr7ODTU/maxresdefault.jpg)](https://youtu.be/dqhfPr7ODTU)

## 项目截图

### Agent管理

<img width="2872" height="1622" alt="image" src="https://github.com/user-attachments/assets/5f21545f-7374-406f-bb66-1c7e145386b5" />

#### 自定义参数
<img width="2872" height="1638" alt="image" src="https://github.com/user-attachments/assets/9ef97c1f-3686-4053-a12b-e22786492b14" />

#### 自定义参数发起对话
<img width="2872" height="1638" alt="image" src="https://github.com/user-attachments/assets/73445742-3635-433b-aff1-6215d54d7996" />

#### 历史记录
<img width="2872" height="1638" alt="image" src="https://github.com/user-attachments/assets/36b99e16-84b3-4276-8253-d4cd02a5b3f5" />


### 应用中心

<img width="2872" height="1622" alt="image" src="https://github.com/user-attachments/assets/883dc859-f866-4b90-9a1d-6bbb60bd03ed" />

#### 智慧题库
<img width="2872" height="1622" alt="image" src="https://github.com/user-attachments/assets/524057f7-7c0d-4712-bc3b-20badfb75213" />

## 功能规划

预研：

Agent通用能力内化，涉及opencv、图片、视频、浏览器、文件处理、附件提取、向量、格式转换、搜索、RAG、GRAPH、WorkFlow、Event、Socket等各组件的能力封装。

前期： 以Agent为主
- Agent管理（已完成）
- 提示词备份 (已完成)
- 问题建议 (高优先级开发中)
- 自定义参数 （已完成）
- Agent日志（对开发者来说日志可视化没有必要，暂缓开发）
- 基于Agent的编排 （开发中）
- 监测 (开发中)

后期： 以业务为主
- 全题型题目录入支持含图片、公式
- 全题型渲染含公式
- 手动、自动组卷
- 交互式答题器
- AI阅卷 （已完成）
- SaaS化
- 用户登录
- rbac权限控制

