-- Your SQL goes here
CREATE TABLE `todos` (
	`id` int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`user_id` int(11) NOT NULL,
	`status` tinyint(1) NOT NULL,
	`title` varchar(50) NOT NULL,
	`description` varchar(255) DEFAULT NULL,
	`target_date` datetime DEFAULT NULL,
	`created_at` datetime NOT NULL,
	`updated_at` datetime DEFAULT NULL
);
