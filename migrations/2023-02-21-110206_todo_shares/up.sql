-- Your SQL goes here
CREATE TABLE `todo_shares` (
	`id` int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`todo_id` int(11) NOT NULL,
	`user_id` int(11) NOT NULL,
	`is_accept` tinyint(1) DEFAULT '0' NOT NULL,
	`is_deleted` tinyint(1) DEFAULT '0' NOT NULL,
	`created_at` datetime NOT NULL,
	`deleted_at` datetime DEFAULT NULL
);
