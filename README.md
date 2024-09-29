what has been done till now 

1. Create a Rust project in Actix-Web
2. Connect it with MongoDB
3. Make a ‘/register‘ Endpoint for registering any user details with -> Username,
Password, Email, etc…
Conditions:
1. If username already there, should not be create
2. Password should be encrypted
3. Send an email to the user once the Registration is completed.
Like, Thanks for Registering our API.
4. Make ‘/login’ api -> Username, Password are the parameters. Needs to
generate a JWT token with all the user details.
5. /forgot-password to change the new password. Get old password and New
password as parameters.
Condition:
1. Old password should not be the same as a new password.
6. Every user has three roles. 1. Student 2. Teacher 3. Admin
7. Create a collection in MongoDB named MarkList
8. /add-marks -> Only teachers can access this endpoint to add students marks.
Only 2 subjects with studentID are the fields
Condition :
1. Once the teacher finishes adding the mark the student has to get
an email, Like, You teacher Banu, added your marks.
9. /edit-mark -> Only teachers can access this endpoint to edit any student's
marks.
    Condition :
    1. Once the teacher finish the edit the mark the student have to get
    email, Like, You teacher Banu, edited your marks in English subject
    to 60 -> 90.. etc

10. /get-results    done
11. /get-students   done

    

....problem i m facing currently
i)sending confirmation mails due to some setting else wise everything  in the project is done  
