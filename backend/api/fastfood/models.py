# This is an auto-generated Django model module.
# You'll have to do the following manually to clean this up:
#   * Rearrange models' order
#   * Make sure each model has one field with primary_key=True
#   * Make sure each ForeignKey and OneToOneField has `on_delete` set to the desired behavior
#   * Remove `managed = False` lines if you wish to allow Django to create, modify, and delete the table
# Feel free to rename the models, but don't rename db_table values or field names.
from django.db import models
from django.contrib.auth.models import User


class Restaurants(models.Model):
    restaurant_id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=280)
    google_location = models.CharField(max_length=100)
    creator = models.ForeignKey(User, models.CASCADE)

    class Meta:
        db_table = 'restaurants'

class Menuentries(models.Model):
    food_id = models.AutoField(primary_key=True)
    restaurant = models.ForeignKey(Restaurants, on_delete=models.CASCADE)
    creator = models.ForeignKey(User, on_delete=models.CASCADE)
    name = models.CharField(max_length=280)
    description = models.CharField(max_length=280)
    price = models.FloatField()
    calories = models.IntegerField()
    creation_date = models.DateTimeField()
    update_date = models.DateTimeField()

    class Meta:
        db_table = 'menuentries'

class FoodRatings(models.Model):
    food = models.OneToOneField(Menuentries, models.CASCADE, primary_key=True)
    user = models.ForeignKey(User, on_delete=models.CASCADE)
    rating = models.IntegerField()
    comment = models.CharField(max_length=280, blank=True, null=True)
    timestamp = models.DateTimeField()

    class Meta:
        db_table = 'food_ratings'
        unique_together = (('food', 'user'),)

class TrustRatings(models.Model):
    food = models.OneToOneField(Menuentries, models.DO_NOTHING, primary_key=True)
    user = models.ForeignKey(User, on_delete=models.CASCADE)
    trust = models.IntegerField()

    class Meta:
        db_table = 'trust_ratings'
        unique_together = (('food', 'user'),)


