create table captcha_answer (
    id bigserial primary key,
    uuid uuid not null unique default public.next_uuid(),
    answer text not null,
    published timestamp not null default now()
);
