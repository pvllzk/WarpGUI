use super::Language;

pub fn make() -> Language {
    Language {
        couldnt_send: "Couldn't send friend request.".to_string(),
        already_sent: "You've already sent this request.".to_string(),
        add_self: "You cannot add yourself as a friend.".to_string(),
        something_went_wrong: "Something went wrong.".to_string(),
        create_pin: "Create a Pin".to_string(),
        enter_pin: "Enter Pin".to_string(),
        incoming_requests: "Incoming requests".to_string(),
        outgoing_requests: "Outgoing requests".to_string(),
        enter_your_pin: "Enter pin to unlock your account.".to_string(),
        choose_a_pin: "Choose a 4-6 digit pin to secure your account.".to_string(),
        invalid_pin: "Invalid or incorrect pin supplied.".to_string(),
        short_pin: "Your pin must be at least 4 characters.".to_string(),
        checking_account: "Checking account..".to_string(),
        prerelease_warning: "Please remember this is pre-release software and bugs, crashes and restarts are expected.".to_string(),
        create_account: "Create Account".to_string(),
        user_agrees: "I agree".to_string(),
        send_a_reply: "Send a reply..".to_string(),
        create_account_desc:
            "It's free and fast, just tell us what you'd like your username to be.".to_string(),
        choose_username: "Choose username".to_string(),
        chatbar_placeholder: "Say something...".to_string(),
        chat_placeholder: "It's quiet... click here to start this conversation.".to_string(),
        copy_friend_code: "Copy Your Friend Code".to_string(),
        copy_code: "Copy Code".to_string(),
        code_copied: "Copied your code!".to_string(),
        your_friends: "Your Friends".to_string(),
        copied_code: "Friend code copied!".to_string(),
        add_someone: "Add Someone".to_string(),
        add_placeholder: "Warp#a3fdc6..".to_string(),
        request_sent: "Friend request sent!".to_string(),
        invalid_code: "Invalid friend code supplied".to_string(),
        unknown: "Unknown".to_string(),
        location: "Location".to_string(),
        badges: "Badges".to_string(),
        save_status: "Save Status".to_string(),
        status_placeholder: "Some status message...".to_string(),
        friends: "Friends".to_string(),
        edit_profile: "Edit Profile".to_string(),
        about: "About".to_string(),
        no_about_message: "No about message set yet...".to_string(),
        developement: "Development".to_string(),
        search: "Search".to_string(),
        favorites: "Favorites".to_string(),
        new_chat: "New Chat".to_string(),
        chats: "Chats".to_string(),
        no_active_chats: "No active chats, yet...".to_string(),
        start_one: "Start one".to_string(),
    }
}
