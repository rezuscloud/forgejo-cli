-dash =
    { IS_MINIMAL() ->
        [yes] -
       *[no] —
    }
msg-whoami = 当前已登录 { $name }@{ $host }
msg-auth-login-oauth_unsupported =
    您安装的 fj 不支持对 { $host_domain } 使用 `login`

    请访问 { $applications_url }
    创建令牌，并使用 `fj auth add-key` 登录
msg-auth-login-canceled = 登录已取消
msg-auth-login-browser_success = 已认证！请关闭此标签页并返回终端。
msg-auth-login-browser_failure = 认证失败。
msg-auth_logout-success = 已退出登录 { $host }
msg-auth_logout-already_signed_out = 尚未登录 { $host }
msg-auth-use_ssh-not-logged-in = 未登录 { $host }
msg-auth-use_ssh-enabled = 现在将默认对 { $host } 使用 SSH
msg-auth-use_ssh-disabled = 将不再默认对 { $host } 使用 SSH
msg-auth-use_ssh-already_enabled = 已默认对 { $host } 使用 SSH
msg-auth-use_ssh-already_disabled = 尚未默认对 { $host } 使用 SSH
msg-auth-add_key-prompt = 新密钥：
msg-auth-add_key-already_exists = { $host } 的密钥已存在
msg-auth-list-none = 无登录记录。
msg-actions-variable-create-already_exists = 变量已存在，请传递 --force 以替换它。
msg-actions-variable-create-already_exists_forced = 变量已存在，正在更新。
msg-actions-variable-delete-success = 变量 { $name } 已删除。
msg-actions-dispatch-success =
    已在 { $ref } 中调度工作流 { $name }，包含 { $n_inputs ->
        [one] 1 个输入
       *[other] { $n_inputs } 个输入
    }。
msg-org-list-no_results = 无结果。
msg-org-list-page_number = 第 { $page } 页，共 { $total } 页
msg-org-view-org_name =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $name }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $name }){ STYLE("reset") }
    }
msg-org-view-visibility =
    { $visibility ->
        [public] 公开
        [limited] 受限
       *[private] 私有
    }
msg-org-view-member_count =
    { $member_count ->
        [one] { STYLE("bold") }1{ STYLE("reset") } 个成员
       *[other] { STYLE("bold") }{ $member_count }{ STYLE("reset") } 个成员
    }
msg-org-view-team_count =
    { $team_count ->
        [one] { STYLE("bold") }1{ STYLE("reset") } 个团队
       *[other] { STYLE("bold") }{ $team_count }{ STYLE("reset") } 个团队
    }
msg-org-create-invalid_character =
    组织名称只能包含字母数字字符、短横线、下划线或句点。
      如果您希望名称包含其他字符，请尝试设置 --full-name 标志
msg-org-create-invalid_starting_character =
    组织名称只能以字母数字字符开头。
      如果您希望名称以其他字符开头，请尝试设置 --full-name 标志
msg-org-create-invalid_ending_character =
    组织名称只能以字母数字字符结尾。
      如果您希望名称以其他字符结尾，请尝试设置 --full-name 标志
msg-org-create-invalid_consecutive_characters =
    组织名称不能包含连续的非字母数字字符。
      如果您希望在名称中包含此类字符，请尝试设置 --full-name 标志
msg-org-create-success =
    已创建新的{ $visibility ->
        [public] 公开
        [limited] 受限
       *[private] 私有
    }组织 { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $name }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $name }){ STYLE("reset") }
    }
msg-org-members-no_results = 无结果。
msg-org-members-page_number = 第 { $page } 页，共 { $total } 页
msg-org-members-entry =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $username }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $username }){ STYLE("reset") }
    }
msg-org-visibility-public = 您是 { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") } 的公开成员
msg-org-visibility-private = 您是 { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") } 的私有成员
msg-org-visibility-set_public = 您现在已是 { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") } 的公开成员
msg-org-visibility-set_private = 您现在已是 { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") } 的私有成员
msg-org-visibility-not_member = 您不是 { STYLE("bold", "bright-cyan") }{ $org_name }{ STYLE("reset") } 的成员
msg-org-label-add-success = 已创建新标签 { $label }
msg-org-label-edit-success = 已将标签 { $old_label } 更改为 { $label }
msg-org-label-remove-success = 已移除标签 { $label }
msg-org-repo-list-no_results = 无结果。
msg-org-repo-list-page_number = 第 { $page } 页，共 { $total } 页
msg-org-team-view =
    { STYLE("bright-blue", "bold") }{ $name }{ STYLE("reset") } 在组织 { STYLE("bold") }{ $org }{ STYLE("reset") } 中 { $admin ->
        [yes] { -dash } { STYLE("bright-red") }管理员{ STYLE("reset") }
       *[no] { "" }
    }
msg-org-team-view-read_only = 只读：
msg-org-team-view-read_write = 读/写：
msg-org-team-view-perms-wiki = Wiki
msg-org-team-view-perms-ext_wiki = 外部 Wiki
msg-org-team-view-perms-issues = 问题
msg-org-team-view-perms-ext_issues = 外部问题
msg-org-team-view-perms-pulls = 拉取请求
msg-org-team-view-perms-projects = 项目
msg-org-team-view-perms-actions = CI
msg-org-team-view-perms-code = 代码
msg-org-team-view-perms-releases = 发布
msg-org-team-view-perms-packages = 软件包
msg-org-team-create-success =
    已在组织 { STYLE("bold") }{ $org }{ STYLE("reset") } 中创建新的{ $admin ->
        [yes] 管理员
       *[no] { "" }
    }团队 { STYLE("bright-blue", "bold") }{ $name }{ STYLE("reset") }
msg-org-team-delete-confirmation = 您确定要删除 { STYLE("bold") }{ $org }/{ $name }{ STYLE("reset") } 吗？
    .yes =
        是
        是
        Y
        y
    .no =
        否
        否
        N
        n
msg-org-team-repo-list-no_results = 无结果。
msg-org-team-repo-list-page_number = 第 { $page } 页，共 { $total } 页
msg-org-team-repo-add-success = 已将 { STYLE("bold") }{ $org }/{ $repo }{ STYLE("reset") } 添加到团队 { STYLE("bold", "bright_blue") }{ $team }{ STYLE("reset") }
msg-org-team-repo-rm-success = 已将 { STYLE("bold") }{ $org }/{ $repo }{ STYLE("reset") } 从团队 { STYLE("bold", "bright_blue") }{ $team }{ STYLE("reset") } 中移除
msg-org-team-member-list-no_results = 无结果。
msg-org-team-member-list-page_number = 第 { $page } 页，共 { $total } 页
msg-org-team-member-add-success = 已将 { STYLE("bold", "bright-cyan") }{ $user }{ STYLE("reset") } 添加到团队 { STYLE("bold", "bright_blue") }{ $team }{ STYLE("reset") }
msg-org-team-member-rm-success = 已将 { STYLE("bold", "bright-cyan") }{ $user }{ STYLE("reset") } 从团队 { STYLE("bold", "bright_blue") }{ $team }{ STYLE("reset") } 中移除
msg-issue-create-no_templates = { $owner }/{ $repo } 没有任何问题模板
msg-issue-create-templates_required =
    { $owner }/{ $repo } 要求使用模板。
    请使用 `--template <名称>` 选择一个。
msg-issue-create-templates_enabled =
    { $owner }/{ $repo } 使用问题模板。
    请使用 `--template <名称>` 选择一个，
    或使用 `--no-template` 从头编写一个。
msg-issue-create-success = 已创建问题 #{ $number }：{ $title }
msg-issue-view-header =
    { STYLE("yellow") }{ $title } { STYLE("dark-grey") }#{ $number }{ STYLE("reset") }
    作者：{ STYLE("white") }{ $author }{ STYLE("reset") } { -dash } { $state ->
        [open] { STYLE("bright-green") }开启{ STYLE("reset") }
        [closed] { STYLE("bright-red") }已关闭{ STYLE("reset") }
       *[other] $state
    }
msg-issue-view-comment_count =
    { $comments ->
        [one] 1 条评论
       *[other] { $comments } 条评论
    }
msg-issue-search-total =
    { $issues ->
        [one] 1 个问题
       *[other] { $issues } 个问题
    }
msg-issue-search-entry = #{ $number }：{ $title }（作者：{ $author }）
msg-issue-templates-none = 没有问题模板或联系信息。
msg-issue-templates-blank_allowed = 允许使用“--no-template”
msg-issue-templates-blank_not_allowed = 不允许使用“--no-template”
msg-issue-view-comments-comment_header =
    { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $username }{ STYLE("reset") } 说：
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("dark-gray") }({ $username }){ STYLE("reset") } 说：
    }
msg-issue-view-comments-attachments =
    { $attachments ->
        [one] 1 个附件
       *[other] { $attachments } 个附件
    }
msg-issue-edit-title-empty = 标题不能为空
msg-issue-edit-title-no_newlines = 标题不能包含换行符
msg-issue-assign-success =
    已将 { $added ->
        [one] 1 个用户
       *[other] { $added } 个用户
    } 分配到 { $owner }/{ $repo }#{ $number } { $duplicate ->
        [0] { "" }
        [one]
            { $added ->
                [0] （用户已被分配）
               *[other] （1 个用户已被分配）
            }
       *[other]
            { $added ->
                [0] （所有用户已被分配）
               *[other] （{ $duplicate } 个用户已被分配）
            }
    }
msg-issue-unassign-success =
    已从 { $owner }/{ $repo }#{ $number } 取消分配 { $removed ->
        [one] 1 个用户
       *[other] { $removed } 个用户
    } { $duplicate ->
        [0] { "" }
        [one]
            { $removed ->
                [0] （用户未被分配）
               *[other] （1 个用户未被分配）
            }
       *[other]
            { $removed ->
                [0] （所有用户均未被分配）
               *[other] （{ $duplicate } 个用户未被分配）
            }
    }
msg-issue-close-success = 已关闭问题 #{ $number }：“{ $title }”
msg-pr-couldnt_guess = 无法猜测拉取请求编号，请指定
msg-pr-not_found = 找不到 PR
msg-pr-view-header =
    { STYLE("yellow") }{ $title } { STYLE("dark-grey") }#{ $number }{ STYLE("reset") }
    作者：{ STYLE("white") }{ $username }{ STYLE("reset") } { -dash } { $state ->
        [draft] { STYLE("light-grey") }草稿{ STYLE("reset") }
        [open] { STYLE("bright-green") }开启{ STYLE("reset") }
        [merged] { STYLE("bright-magenta") }已合并{ STYLE("reset") }
        [closed] { STYLE("bright-red") }已关闭{ STYLE("reset") }
       *[other] $state
    } { -dash } { STYLE("bright-green") }+{ $additions } { STYLE("bright-red") }-{ $deletions }{ STYLE("reset") }
    { OPT($head_branch) ->
       *[none] 合并到 `{ $base_branch }`
        [some] 从 `{ $head_branch }` 合并到 `{ $base_branch }`
    }
msg-pr-view-comment_count =
    { $comments ->
        [one] 1 条评论
       *[other] { $comments } 条评论
    }
msg-pr-status-merged = { STYLE("bright-magenta") }已合并{ STYLE("reset") }，由 { $merged_by } 于 { DATETIME($created_at, dateStyle: "long", timeStyle: "long") } 操作
msg-pr-status-header =
    { $state ->
        [draft] { STYLE("light-grey") }草稿{ STYLE("reset") } { -dash } 无法合并草稿 PR
        [open]
            { STYLE("bright_green") }开启{ STYLE("reset") } { -dash } { $mergeable ->
               *[yes] 可以合并
                [no] { STYLE("bright-red") }合并冲突{ STYLE("reset") }
            }
        [closed] { STYLE("bright-red") }已关闭{ STYLE("reset") } { -dash } 重新开启以合并
       *[other] 未知
    }
msg-pr-status-entry =
    { $state ->
        [success] { STYLE("bright_green") }成功{ STYLE("reset") }
        [pending] { STYLE("yellow") }待处理{ STYLE("reset") }
        [warning] { STYLE("bright_yellow") }警告{ STYLE("reset") }
        [failure] { STYLE("bright_red") }失败{ STYLE("reset") }
        [error] { STYLE("bright_red") }错误{ STYLE("reset") }
       *[other] 未知
    } { -dash } { $context }
msg-pr-review-list-none = 无评论。
msg-pr-review-list-only_stale = 只有已过时或已驳回的评论，请使用 --all 来显示它们。
msg-pr-review-list-review_header =
    { $review_type ->
        [approved] { STYLE("bright-green") }已批准{ STYLE("reset") }
        [changes-requested] { STYLE("bright-yellow") }请求变更{ STYLE("reset") }
        [comment] { STYLE("bright-yellow") }评论{ STYLE("reset") }
        [pending] { STYLE("light-grey") }待审核{ STYLE("reset") }
       *[other] 未知
    }，来自 { STYLE("bold") }{ $reviewer }{ STYLE("reset") }
    { STYLE("dark-grey") }{ $comments ->
        [one] 1 条评论
       *[other] { $comments } 条评论
    }，发表于 { DATETIME($timestamp, dateStyle: "long", timeStyle: "short") }{ STYLE("reset") } { $state ->
        [stale] { STYLE("bold") }（已过时）{ STYLE("reset") }
        [dismissed] { STYLE("bold") }（已驳回）{ STYLE("reset") }
       *[other] { "" }
    }
msg-pr-review-list-comment_position = 在 { STYLE("bold") }{ $path }：{ $position }{ STYLE("reset") } 中：
msg-pr-review-list-comment_header =
    { STYLE("bold", "bright-cyan") }{ $commenter }{ STYLE("reset") } 评论了 { OPT($resolver) ->
       *[none] { "" }
        [some] （由 { $resolver } 解决）
    }：
msg-pr-create-cross_instance = 无法跨实例创建拉取请求；基础分支位于 { $base_instance }，而头部分支跟踪的是 { $head_instance }
msg-pr-create-success = 已创建拉取请求 #{ $number }：{ $title }
msg-pr-create-agit_success = 已创建拉取请求：{ $title }
msg-pr-create-agit_push_cfg_question =
    是否要设置所需的 git 配置项，
    以便 `git push` 对此 PR 生效？
msg-pr-create-agit_push_cfg_prompt = （y/N/?）
    .yes =
        是
        是
        Y
        y
    .no =
        否
        否
        N
        n
    .help =
        帮助
        帮助
        H
        h
        ？
msg-pr-create-agit_force_push_warning =
    { STYLE("bold") }注意：{ STYLE("reset") }
      AGit PR 不支持 `git push --force[-with-lease]`。
      您可以使用 `git push -o force=true` 代替。
msg-pr-create-agit_push_cfg_help = 这将设置以下配置选项：
msg-pr-merge-commit_title_unsupported-rebase = 变基不支持提交标题
msg-pr-merge-commit_title_unsupported-ff = ff-only 不支持提交标题
msg-pr-merge-commit_title_unsupported-manual = 手动合并不支持提交标题
msg-pr-merge-default_message = 审阅地址：{ $pr_url }
msg-pr-merge-success = 已将 PR #{ $number }“{ $title }”合并到“{ $base_branch }”
msg-pr-checkout-dirty = 无法检出 PR；工作目录存在未提交的更改
msg-pr-checkout-not_fork = 无法获取父仓库，{ $repo } 不是复刻仓库
msg-pr-checkout-success =
    已检出 PR #{ $number }：{ $title }
    { $new_branch ->
       *[yes] 在新分支 { $branch_name } 上
        [no] 已将分支更新到最新提交
    }
msg-pr-search-count =
    { $pull_requests ->
        [one] 1 个拉取请求
       *[other] { $pull_requests } 个拉取请求
    }
msg-pr-search-entry = #{ $number }：{ $title }（作者：{ $author }）
msg-pr-view-diff-volatile = 对差异所做的更改将不会保留
msg-repo-no_host_given = 找不到仓库，未指定主机
msg-repo-no_info_given =
    未指定仓库信息

    如果您尝试对当前目录中的仓库进行操作，请尝试添加一个指向
    Forgejo 实例的远程仓库。如果您有多个远程仓库，请尝试将其中一个设置为
    当前分支的上游。您也可以使用 `--host` 参数显式指定主机。
msg-repo-fallback_host-invalid_url = 警告：`FJ_FALLBACK_HOST` 未设置为有效的 URL
msg-repo-arg_no_owner = 仓库名称格式应为 [HOST/]OWNER/NAME
msg-repo-name_needed = 无法获取仓库名称，请指定
msg-repo-create-remote_exists = 名为“{ $remote_name }”的远程仓库已存在
msg-repo-create-success = 已在 { $url } 创建新仓库
msg-repo-create-detached_head = HEAD 不在任何分支上；无法推送到远程仓库
msg-repo-create-branch_invalid_utf8 = 分支名称包含无效的 UTF-8 字符
msg-repo-fork-conflicting_hosts = 主机 { $host_a } 和 { $host_b } 冲突，请仅指定一个
msg-repo-fork-success = 已将 { $parent_owner }/{ $parent_name } 复刻到 { $fork_name }
msg-repo-migrate-git_only = 从 `git` 服务迁移不支持除 LFS 之外的迁移项目。请指定其他服务或移除包含的项目
msg-repo-migrate-username_prompt = 用户名：
msg-repo-migrate-password_prompt = 密码：
msg-repo-migrate-token_prompt = 令牌：
msg-repo-migrate-migrating = 正在迁移...
msg-repo-migrate-success = 完成！在线查看：{ $url }
msg-repo-view-name = { $repo_name }
msg-repo-view-is_fork = 派生自 { $parent }
msg-repo-view-is_mirror = 镜像自 { $mirror_of }
msg-repo-view-primary_language = 主要语言为 { $language }
msg-repo-view-stars =
    { $stars ->
        [one] 1 颗星
       *[other] { $stars } 颗星
    }
msg-repo-view-watching = { $watching } 人关注中
msg-repo-view-forks =
    { $forks ->
        [one] 1 个复刻
       *[other] { $forks } 个复刻
    }
msg-repo-view-issues =
    { $issues ->
        [one] 1 个问题
       *[other] { $issues } 个问题
    }
msg-repo-view-prs =
    { $pull_requests ->
        [one] 1 个 PR
       *[other] { $pull_requests } 个 PR
    }
msg-repo-view-releases =
    { $releases ->
        [one] 1 个发布
       *[other] { $releases } 个发布
    }
msg-repo-view-external_tracker = 问题跟踪器位于 { $url }
msg-repo-view-url = 在线查看：{ $url }
msg-repo-readme-none = 仓库没有 README
msg-repo-clone-preparing = { "   " }正在准备...
msg-repo-clone-downloading = { " " }正在下载... { NUMBER($percent, maximumFractionDigits: 2) }%（{ NUMBER($size, maximumFractionDigits: 2) }{ $units }）
msg-repo-clone-resolving = { "   " }正在解析... { NUMBER($percent, maximumFractionDigits: 2) }%
msg-repo-clone-finishing_up = 正在完成...
msg-repo-clone-success = 已将 { $repo } 克隆到 { $path }
msg-repo-star-success = 已收藏 { $owner }/{ $repo }！
msg-repo-unstar-success = 已取消收藏 { $owner }/{ $repo }！
msg-repo-delete-confirmation_prompt = 您确定要删除 { $owner }/{ $name } 吗？（y/N）
    .yes =
        是
        是
        Y
        y
    .no =
        否
        否
        N
        n
msg-repo-delete-success = 已删除 { $owner }/{ $repo }
msg-repo-delete-cancelled = 未删除
msg-repo-label-view-archived = （已归档）
msg-repo-label-view-no_description = （无描述）
msg-repo-label-create-success = 成功创建标签 { $label }
msg-repo-label-delete-success = 成功删除标签 { $label }
msg-repo-label-edit-success = 已编辑标签：{ $label }
msg-user-search-page_zero = 没有第 0 页
msg-user-search-fail = 搜索失败
msg-user-search-none = 没有用户匹配该查询
msg-user-search-page_too_high =
    { $total_pages ->
        [one] 只有 1 页
       *[other] 只有 { $total_pages } 页
    }
msg-user-search-footer =
    显示第 { STYLE("bold") }{ $first_index }{ -dash }{ $last_index }{ STYLE("reset") } 条结果，共 { STYLE("bold") }{ $total_results }{ STYLE("reset") } 条（{ $page }/{ $total_pages }）
    { $more ->
        [yes] 使用 --page 标志查看更多
       *[no] { "" }
    }
msg-user-view-header =
    { STYLE("bright-cyan", "bold") }{ $username }{ STYLE("reset") } { OPT($pronouns) ->
       *[none] { "" }
        [some] { STYLE("light-grey") } { -dash } { STYLE("bold") }{ $pronouns }{ STYLE("reset") }
    }
    { $followers ->
        [one] { STYLE("bold") }1{ STYLE("reset") } 个粉丝
       *[other] { STYLE("bold") }{ $followers }{ STYLE("reset") } 个粉丝
    } { -dash } { STYLE("bold") }{ $following }{ STYLE("reset") } 人关注
    { OPT($website) ->
       *[none]
            { OPT($email) ->
               *[none] { "" }
                [some] { STYLE("bold") }{ $email }{ STYLE("reset") }
            }
        [some]
            { OPT($email) ->
               *[none] { STYLE("bold") }{ $website }{ STYLE("reset") }
                [some] { STYLE("bold") }{ $website }{ STYLE("reset") } { -dash } { STYLE("bold") }{ $email }{ STYLE("reset") }
            }
    }
msg-user-view-joined_on = 于 { STYLE("bold") }{ DATETIME($joined, dateStyle: "medium") }{ STYLE("reset") } 加入
msg-user-follow-success = 已关注 { $username }
msg-user-unfollow-success = 已取消关注 { $username }
msg-user-following-none-other = { $user } 没有关注任何人
msg-user-following-none-self = 您还没有关注任何人
msg-user-following-other = { $user } 关注的人：
msg-user-following-self = 您关注的人：
msg-user-followers-none-other = { $user } 没有粉丝
msg-user-followers-none-self = 您还没有粉丝 :(
msg-user-followers-other = 关注 { $user } 的人：
msg-user-followers-self = 关注您的人：
msg-user-block-success = 已屏蔽 { $user }
msg-user-unblock-success = 已解除屏蔽 { $user }
msg-user-repos-none-starred-other = { $name } 未收藏任何仓库
msg-user-repos-none-starred-self = 您还没有收藏任何仓库
msg-user-repos-none-other = { $name } 没有任何仓库
msg-user-repos-none-self = 您没有任何仓库
msg-user-repos-list_footer =
    显示第 { STYLE("bold") }{ $first_index }{ -dash }{ $last_index }{ STYLE("reset") } 条结果，共 { STYLE("bold") }{ $total_results }{ STYLE("reset") } 条（{ $page }/{ $total_pages }）
    { $more ->
        [yes] 使用 --page 标志查看更多
       *[no] { "" }
    }
msg-user-orgs-none-other = { $user } 还不是任何组织的成员
msg-user-orgs-none-self = 您还不是任何组织的成员
msg-user-orgs-count =
    { $organizations ->
        [one] 1 个组织
       *[other] { $organizations } 个组织
    }
msg-activity-created_fork = { STYLE("bold") }{ $actor }{ STYLE("reset") } 将仓库 { STYLE("bold", "yellow") }{ $parent_repo_name }{ STYLE("reset") } 复刻到 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") }
msg-activity-created_mirror = { STYLE("bold") }{ $actor }{ STYLE("reset") } 创建了镜像 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") }
msg-activity-created_repo = { STYLE("bold") }{ $actor }{ STYLE("reset") } 创建了仓库 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") }
msg-activity-renamed_repo = { STYLE("bold") }{ $actor }{ STYLE("reset") } 将仓库从 { STYLE("bold", "yellow") }"{ $old_name }"{ STYLE("reset") } 重命名为 { STYLE("bold", "yellow") }{ $new_name }{ STYLE("reset") }
msg-activity-starred_repo = { STYLE("bold") }{ $actor }{ STYLE("reset") } 收藏了仓库 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") }
msg-activity-watched_repo = { STYLE("bold") }{ $actor }{ STYLE("reset") } 关注了仓库 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") }
msg-activity-pushed_commit = { STYLE("bold") }{ $actor }{ STYLE("reset") } 推送到 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") } 上的 { STYLE("bold", "bright-cyan") }{ $branch }{ STYLE("reset") }
msg-activity-created_issue = { STYLE("bold") }{ $actor }{ STYLE("reset") } 开启了问题 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-created_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 创建了拉取请求 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-transferred_repo = { STYLE("bold") }{ $actor }{ STYLE("reset") } 将仓库 { STYLE("bold", "yellow") }"{ $old_name }"{ STYLE("reset") } 转移至 { STYLE("bold", "yellow") }{ $new_name }{ STYLE("reset") }
msg-activity-pushed_tag = { STYLE("bold") }{ $actor }{ STYLE("reset") } 将标签 { STYLE("bold", "bright_cyan") }{ $tag_name }{ STYLE("reset") } 推送到 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") }
msg-activity-commented_issue = { STYLE("bold") }{ $actor }{ STYLE("reset") } 评论了问题 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-merged_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 合并了拉取请求 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-closed_issue = { STYLE("bold") }{ $actor }{ STYLE("reset") } 关闭了问题 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-reopened_issue = { STYLE("bold") }{ $actor }{ STYLE("reset") } 重新开启了问题 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-closed_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 关闭了 PR { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-reopened_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 重新开启了 PR { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-deleted_tag = { STYLE("bold") }{ $actor }{ STYLE("reset") } 从 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") } 删除了标签 { STYLE("bold", "bright_cyan") }{ $tag_name }{ STYLE("reset") }
msg-activity-deleted_branch = { STYLE("bold") }{ $actor }{ STYLE("reset") } 从 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") } 删除了分支 { STYLE("bold", "bright_cyan") }{ $branch }{ STYLE("reset") }
msg-activity-approved_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 已批准 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-rejected_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 对 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") } 提出了更改建议
msg-activity-commented_pr = { STYLE("bold") }{ $actor }{ STYLE("reset") } 评论了拉取请求 { STYLE("bold", "yellow") }{ $repo_name }#{ $number }{ STYLE("reset") }
msg-activity-created_release = { STYLE("bold") }{ $actor }{ STYLE("reset") } 在 { STYLE("bold", "yellow") }{ $repo_name }{ STYLE("reset") } 上创建了发布 { STYLE("bold", "bright_cyan") }{ $release_name }{ STYLE("reset") }
msg-user-edit-name-removal_hint = 使用 --unset 从您的个人资料中移除姓名
msg-user-edit-pronouns-removal_hint = 使用 --unset 从您的个人资料中移除代词
msg-user-edit-location-removal_hint = 使用 --unset 从您的个人资料中移除位置信息
msg-user-edit-website-removal_hint = 使用 --unset 从您的个人资料中移除网站
msg-user-key-list-count = 密钥总数：{ $keys }
msg-user-key-list-header = { STYLE("bold") }密钥 { STYLE("bright-magenta") }{ $id }{ STYLE("reset") }
msg-user-key-list-title = { STYLE("bold") }标题：{ STYLE("reset") }       { STYLE("bright-cyan") }{ $title }{ STYLE("reset") }
msg-user-key-list-created_at = { STYLE("bold") }创建时间：{ STYLE("reset") }  { STYLE("bright-cyan") }{ DATETIME($created_at, dateStyle: "short", timeStyle: "medium") }{ STYLE("reset") }
msg-user-key-list-type = { STYLE("bold") }类型：{ STYLE("reset") }        { STYLE("bright-cyan") }{ $key_type }{ STYLE("reset") }
msg-user-key-list-fingerprint = { STYLE("bold") }指纹：{ STYLE("reset") } { STYLE("bright-cyan") }{ $fingerprint }{ STYLE("reset") }
msg-user-key-delete-success = 已成功删除 ID 为 { $id } 的密钥
msg-user-key-upload-home_not_found = 无法定位主目录。请为密钥文件提供显式路径。
msg-user-key-upload-keys_not_found = 未找到密钥。
msg-user-key-upload-confirm_key_file_prompt =
    猜测的密钥文件：{ $path }
    看起来是否正确？
    .yes =
        是
        是
        Y
        y
    .no =
        否
        否
        N
        n
msg-user-key-add-file_unconfirmed = 用户未确认猜测的密钥文件。
msg-user-key-add-unexpected_extension =
    “{ $path }”不以“.pub”结尾。您确定这不是私钥吗？
     如果您仍要继续，请添加 --force。
msg-user-key-add-invalid_key =
    “{ $path }”看起来像私钥或无效数据！
     如果您仍要继续，请添加 --force。
msg-user-key-add-no_title = 无法猜测密钥标题，请明确提供一个并检查您的密钥文件。
msg-user-key-upload-confirm_key_title_prompt =
    猜测的标题：{ STYLE("bright-cyan") }{ $title }{ STYLE("reset") }
    看起来是否正确？
    .yes =
        是
        是
        Y
        y
    .no =
        否
        否
        N
        n
msg-user-key-add-title_unconfirmed = 用户未确认猜测的标题。
msg-user-key-add-success = 密钥创建成功！
msg-user-gpg-list-count = 密钥总数：{ $keys }
msg-user-gpg-list-header = { STYLE("bold") }密钥 { STYLE("bright-magenta") }{ $id }{ STYLE("reset") }
msg-user-gpg-list-key_id = { STYLE("bold") }密钥 ID：{ STYLE("reset") }              { STYLE("bright-cyan") }{ $key_id }{ STYLE("reset") }
msg-user-gpg-list-can_sign =
    { STYLE("bold") }可签名：{ STYLE("reset") }            { $can_sign ->
        [yes] { STYLE("bright-green") }是{ STYLE("reset") }
       *[no] { STYLE("bright-red") }否{ STYLE("reset") }
    }
msg-user-gpg-list-can_encrypt_comms =
    { STYLE("bold") }可加密通信：{ STYLE("reset") }   { $can_encrypt_comms ->
        [yes] { STYLE("bright-green") }是{ STYLE("reset") }
       *[no] { STYLE("bright-red") }否{ STYLE("reset") }
    }
msg-user-gpg-list-can_encrypt_storage =
    { STYLE("bold") }可加密存储：{ STYLE("reset") } { $can_encrypt_storage ->
        [yes] { STYLE("bright-green") }是{ STYLE("reset") }
       *[no] { STYLE("bright-red") }否{ STYLE("reset") }
    }
msg-user-gpg-list-can_certify =
    { STYLE("bold") }可认证：{ STYLE("reset") }         { $can_certify ->
        [yes] { STYLE("bright-green") }是{ STYLE("reset") }
       *[no] { STYLE("bright-red") }否{ STYLE("reset") }
    }
msg-user-gpg-list-verified =
    { STYLE("bold") }已验证：{ STYLE("reset") }            { $verified ->
        [yes] { STYLE("bright-green") }是{ STYLE("reset") }
       *[no] { STYLE("bright-red") }否{ STYLE("reset") }
    }
msg-user-gpg-list-email =
    { STYLE("bright-cyan") }{ $email }{ STYLE("reset") } { $verified ->
        [yes] 已验证
       *[no] 未验证
    }
msg-user-gpg-list-subkey = { STYLE("bold") }子密钥 { STYLE("bright-magenta") }{ $id }{ STYLE("reset") }：
msg-user-gpg-upload-exporting = 正在导出密钥...
msg-user-gpg-upload-export_failed =
    导出密钥失败。{ OPT($status_code) ->
       *[none] { "" }
        [some] GPG 状态：{ $status_code }
    }
msg-user-gpg-upload-success = 密钥添加成功！
msg-user-gpg-verify-fetching_token = 正在获取验证令牌...
msg-user-gpg-verify-signing_token = 正在使用密钥“{ $key_name }”签署验证令牌...
msg-user-gpg-verify-signing_failed =
    签署验证令牌失败。{ OPT($status_code) ->
       *[none] { "" }
        [some] GPG 状态：{ $status_code }
    }
msg-user-gpg-verify-key_to_verify = 正在验证此密钥：
msg-user-gpg-verify-success = 验证成功！
msg-user-gpg-delete-confirmation_prompt = 删除 GPG 密钥将导致该密钥签名的所有提交变为未验证！是否继续？
    .yes =
        是
        是
        Y
        y
    .no =
        否
        否
        N
        n
msg-user-gpg-delete-unconfirmed = 用户中止了进程。
msg-user-gpg-delete-success = ID 为 { $id } 的密钥已成功删除。
msg-release-create-must_specify_tag = 必须使用 `--tag` 或 `--create-tag` 选择标签
msg-release-create-tag_flags_conflict = “--tag”和“--create-tag”互斥，请仅选择其中一个
msg-release-create-success = 已创建发布 { $name }
msg-release-list-entry =
    { $name } { $state ->
       *[neither] { "" }
        [draft] （草稿）
        [prerelease] （预发布）
        [both] （草稿，预发布）
    }
msg-release-view-header =
    { $name }
    作者：{ $author }，发布于 { DATETIME($created_at, dateStyle: "long") }
msg-release-asset-create-success = 已将附件“{ $asset }”添加到 { $release }
msg-release-asset-delete-success = 已从 { $release } 中移除附件“{ $asset }”
msg-release-asset-download-success =
    { OPT($file) ->
       *[none] 已下载 { $asset }
        [some] 已将 { $asset } 下载到 { $file }
    }
msg-tag-create-success = 已创建标签 { $name }
msg-tag-delete-success = 已删除标签 { $name }
msg-version-update_check-hint = 使用 `fj version --check` 检查新版本
msg-version-update_check-current = 已是最新！
msg-version-update_check-behind =
    新版本可用：{ $new_version }
    获取地址：{ $url }
msg-version-update_check-ahead = 您领先于最新的已发布版本
msg-wiki-clone-success = 已将 { $repo } 的 wiki 克隆到 { $path }
