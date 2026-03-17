from js import Response

async def on_fetch(request, env):
    # Mock data analysis for COMELEC meeting attendance
    attendance_data = [10, 15, 12, 18, 14]
    average = sum(attendance_data) / len(attendance_data)
    
    report = f"Average Attendance for 2026 Sessions: {average}"
    return Response.new(report)
