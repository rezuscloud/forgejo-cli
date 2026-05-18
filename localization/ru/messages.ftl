msg-auth-add_key-prompt = новый ключ:
msg-auth-add_key-already_exists = ключ для { $host } уже добавлен
msg-actions-variable-create-already_exists = переменная уже существует. Добавьте --force, чтобы заменить её.
msg-actions-variable-create-already_exists_forced = переменная уже существует и будет изменена.
msg-actions-variable-delete-success = Переменная { $name } была удалена.
msg-org-list-page_number = Страница { $page } из { $total }
msg-org-view-visibility =
    { $visibility ->
        [public] Публичная
        [limited] Ограниченная
       *[private] Частная
    }
msg-org-create-success =
    создана новая { $visibility ->
        [public] публичная
        [limited] ограниченная
       *[private] приватная
    } организация { OPT($full_name) ->
       *[none] { STYLE("bold", "bright-cyan") }{ $name }{ STYLE("reset") }
        [some] { STYLE("bold", "bright-cyan") }{ $full_name }{ STYLE("reset") } { STYLE("light-gray") }({ $name }){ STYLE("reset") }
    }
msg-org-label-add-success = Создана новая метка { $label }
msg-org-label-edit-success = Метка { $old_label } изменена на { $label }
msg-org-label-remove-success = Метка { $label } была удалена
msg-org-repo-list-page_number = Страница { $page } из { $total }
msg-org-team-view-read_only = Только чтение:
msg-org-team-view-read_write = Чтение и запись:
msg-org-team-view-perms-issues = Задачи
msg-org-team-view-perms-ext_issues = Внешние задачи
msg-org-team-view-perms-releases = Выпуски
msg-org-team-repo-list-page_number = Страница { $page } из { $total }
msg-org-team-member-list-page_number = Страница { $page } из { $total }
msg-issue-create-success = создана задача #{ $number }: { $title }
msg-issue-edit-title-empty = название не может быть пустым
msg-issue-edit-title-no_newlines = название должно быть в одну строку
msg-auth-login-canceled = Вход был отменён
msg-auth-login-browser_success = Вход успешен! Закрывайте эту вкладку и возвращайтесь в консоль.
msg-auth-login-browser_failure = Не удалось войти.
msg-auth-list-none = Нет учётных записей.
msg-repo-migrate-username_prompt = Имя пользователя:
msg-repo-migrate-password_prompt = Пароль:
msg-repo-migrate-token_prompt = Ключ:
msg-repo-view-is_mirror = Зеркало { $mirror_of }
msg-repo-view-primary_language = Основной язык — { $language }
msg-repo-view-is_fork = Ответвление { $parent }
msg-repo-label-view-archived = (архивирована)
msg-repo-label-view-no_description = (без описания)
msg-user-followers-none-self = На вас никто не подписан :(
