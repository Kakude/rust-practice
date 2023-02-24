-- Your SQL goes here
CREATE TABLE `users` (
	`id` int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`email` varchar(255) NOT NULL UNIQUE,
	`name` varchar(50) NOT NULL,
	`password` varchar(255) NOT NULL,
	`created_at` datetime NOT NULL,
	`updated_at` datetime DEFAULT NULL
);
