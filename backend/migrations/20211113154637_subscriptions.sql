-- Add migration script here
create table if not exists subscriptions (
    subscriber uuid,
    subscribed_to uuid,
    foreign key (subscriber) references users (id),
    foreign key (subscribed_to) references users (id),
    primary key (subscriber, subscribed_to)
)