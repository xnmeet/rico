// Service definition for UserService
service UserService {
    // Method to get a user by ID
    User getUser(1: i32 userId),

    // Method to create a new user
    User createUser(1: User newUser),

    // Method to update an existing user's information
    void updateUser(1: User updatedUser) throws (1: UserNotFoundException e),

    // Method to delete a user by ID
    void deleteUser(1: i32 userId),

    // Method to list all users
    list<User> listUsers()
}

// Service definition for OrderService
service OrderService {
    // Method to place a new order
    Order placeOrder(1: Order newOrder),

    // Method to get an order by its ID
    Order getOrder(1: i32 orderId),

    // Method to update the details of an existing order
    void updateOrder(1: Order updatedOrder),

    // Method to cancel an order by its ID
    void cancelOrder(1: i32 orderId),

    // Method to list all orders for a specific user
    list<Order> listOrders(1: i32 userId)
}

// Service definition for ProductService
service ProductService {
    // Method to get a product by its ID
    Product getProduct(1: i32 productId),

    // Method to add a new product
    Product addProduct(1: Product newProduct),

    // Method to update a product's information
    void updateProduct(1: Product updatedProduct),

    // Method to delete a product by its ID
    oneway void deleteProduct(1: i32 productId),

    // Method to search for products by keywords
    list<Product> searchProducts(1: string keywords)
}(api.type="post",api.version="2")

// Service definition for NotificationService
service NotificationService extends AppService {
    // Method to send a notification to a user
    void sendNotification(1: Notification notification, 1: i32 userId),

    // Method to mark a notification as read
    void markNotificationAsRead(1: i32 notificationId) ,

    // Method to get all unread notifications for a user
    list<Notification> getUnreadNotifications(1: i32 userId),

    // Method to delete a notification by its ID
    void deleteNotification(1: i32 notificationId)
}
